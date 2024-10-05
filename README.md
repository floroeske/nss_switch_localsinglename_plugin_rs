# libnss_localsinglename

## Description

A small module for the NSS (Name Service Switch) to lookup hostnames as singlenames with mdns. Probably not advised to do so. But I think it's convenient. More of an experiment.

This NSS module just takes 'myhostname' and does a new look-up for 'myhostname.local'.

## Installation

```
./install.sh
```

## Configuration

Add localsinglename to your hosts entry in your /etc/nsswitch.conf file.

e.g.
```
hosts:          files localsinglename mdns4_minimal [NOTFOUND=return] dns
```