package Simple::HTTP;
use strict;
use warnings;
use utf8;

use POSIX qw(EINTR);
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

sub run {
    my ($host, $port) = @_;

	infolog("--> listening to %s:%d\n", $host, $port);

    my $listen = IO::Socket::INET->new(
        Listen => SOMAXCONN,
        LocalPort => $port,
        LocalAddr => $host,
        Proto     => 'tcp',
        ReuseAddr => 1,
    );

    while (1) {
        if (my ($conn, $peer) = $listen->accept) {
            infolog(".");
            my ($peerport, $peerhost) = unpack_sockaddr_in $peer;
            my $peeraddr = inet_ntoa($peerhost);

            my $pid = fork;
            unless (defined $pid) {
                errlog("fork failed:$!");
                next;
            }
            unless ($pid) {
                # child process
                $listen->close;
                handle_connection($conn);
                $conn->close;
                exit(0);
            }

            $conn->close;
        }
    }
}

1;
