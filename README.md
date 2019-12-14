# passthrough-dns

A simple dns server that generates A records on the fly. It parses the left-most name label as a hyphenated IP address and passes it through to the A record data.

This is used in a case where I want to use a wildcard SSL certificate on ephemeral machines without messing with floating IPs or a load balancer.
Much like described here: https://blog.filippo.io/how-plex-is-doing-https-for-all-its-users/

## For example:
```
0-0-0-0.a.example.com.	3600	IN	A	0.0.0.0
255-111-255-222.a.example.com. 3600 IN	A	255.111.255.222
```

## Starting the server:
```bash
passthrough-dns ns.example.com hostmaster.example.com -n a.example.com -n b.example.com
```
