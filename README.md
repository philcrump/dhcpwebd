# dhcpwebd

A small HTTPS server hosting a simple System + DHCP leases status page for an OpenBSD router.

The data refreshes once per second, and button for each lease will trigger a server-side ping test to the listed hosts.

<p align="center">
  <img src="/docs/Screenshot.png" width="70%" />
</p>

## Configuration

### `application_config.toml`

```toml
[web]
host = "0.0.0.0:10443"

[leases]
#filepath = "/var/db/dhcpd.leases"
filepath = "leases.example"
```

`leases.filepath` should be modified for the production system.


## License

Unless otherwise specified, all materials of this project are licensed under the BSD 3-clause License (New BSD License).

## Authors

* Phil Crump <phil@philcrump.co.uk>
