#!/usr/bin/perl
use strict;
use warnings;
use utf8;

use Getopt::Long;

use Simple::HTTP;

my ($host, $port) = ('127.0.0.1', '10020');
GetOptions(
    'h=s' => \$host,
    'p=s' => \$port,
) or die;

Simple::HTTP::run($host, $port);
