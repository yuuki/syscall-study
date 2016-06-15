#!/usr/bin/perl
use strict;
use warnings;
use utf8;

use Getopt::Long;

use Simple::HTTP;

my ($host, $port, $workers) = ('127.0.0.1', '10020', 5);
GetOptions(
    'h=s' => \$host,
    'p=s' => \$port,
    'n=s' => \$workers,
) or die;

Simple::HTTP::run($host, $port, $workers);
