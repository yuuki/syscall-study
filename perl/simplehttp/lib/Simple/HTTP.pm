package Simple::HTTP;
use strict;
use warnings;
use utf8;

use POSIX qw(:sys_wait_h);
use IO::Socket::INET;
use Socket qw(IPPROTO_TCP TCP_NODELAY);

sub infolog {
    my ($format, @opts) = @_;
    print sprintf($format, @opts);
}

sub errlog {
    my ($format, @opts) = @_;
    print STDERR sprintf($format, @opts);
}

sub handle_connection {
    my ($conn) = @_;

    my $data;
    $conn->recv($data, 1024);
    $conn->send("HTTP/1.0 200 OK\n");
}

sub worker_handler {
    my ($listen) = @_;

    while (1) {
        if (my ($conn, $peer) = $listen->accept) {
            infolog(".");
            my ($peerport, $peerhost) = unpack_sockaddr_in $peer;
            my $peeraddr = inet_ntoa($peerhost);

            handle_connection($conn);
            $conn->close;
        }
    }
}

sub run {
    my ($host, $port, $workers) = @_;

	infolog("--> listening to %s:%d\n", $host, $port);

    my $listen = IO::Socket::INET->new(
        Listen => SOMAXCONN,
        LocalPort => $port,
        LocalAddr => $host,
        Proto     => 'tcp',
        ReuseAddr => 1,
    );

    my $worker_pids = [];
    for (my $i = 0; $i < $workers; $i++) {
        my $pid = fork;
        unless (defined $pid) {
            errlog("fork failed:$!");
            next;
        }
        unless ($pid) {
            worker_handler($listen);
        }

        push @$worker_pids, $pid;
    }

    $SIG{INT} = sub {
        for (my $i = 0; $i < $workers; $i++) {
            kill $worker_pids->[$i];
        }
        while (wait() > 0) { };
        exit 0;
    };

    while (1) {
        sleep 1;
    }
}

1;
