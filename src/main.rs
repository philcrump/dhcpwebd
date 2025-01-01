
use std::{
    fs,
    sync::Arc,
};

use config::Config;

use poem::{
    endpoint::StaticFilesEndpoint,
    EndpointExt,
    listener::{Listener, RustlsCertificate, RustlsConfig, TcpListener},
    Route,
    Server,
};
use tokio::time::Duration;
use serde::{Serialize, Deserialize};
use hypertext::{html_elements, GlobalAttributes, Renderable};

use dhcpd_parser::leases::LeasesMethods;

use systemstat::{System, Platform, saturating_sub_bytes};

use rand::random;
use chrono::{NaiveDateTime, Utc};
mod timeago_short;

const CONFIG_DIRPATH: &str = "./application_config.toml";

const CERT: &str = r#"
-----BEGIN CERTIFICATE-----
MIIEADCCAmigAwIBAgICAcgwDQYJKoZIhvcNAQELBQAwLDEqMCgGA1UEAwwhcG9u
eXRvd24gUlNBIGxldmVsIDIgaW50ZXJtZWRpYXRlMB4XDTE2MDgxMzE2MDcwNFoX
DTIyMDIwMzE2MDcwNFowGTEXMBUGA1UEAwwOdGVzdHNlcnZlci5jb20wggEiMA0G
CSqGSIb3DQEBAQUAA4IBDwAwggEKAoIBAQCpVhh1/FNP2qvWenbZSghari/UThwe
dynfnHG7gc3JmygkEdErWBO/CHzHgsx7biVE5b8sZYNEDKFojyoPHGWK2bQM/FTy
niJCgNCLdn6hUqqxLAml3cxGW77hAWu94THDGB1qFe+eFiAUnDmob8gNZtAzT6Ky
b/JGJdrEU0wj+Rd7wUb4kpLInNH/Jc+oz2ii2AjNbGOZXnRz7h7Kv3sO9vABByYe
LcCj3qnhejHMqVhbAT1MD6zQ2+YKBjE52MsQKU/xhUpu9KkUyLh0cxkh3zrFiKh4
Vuvtc+n7aeOv2jJmOl1dr0XLlSHBlmoKqH6dCTSbddQLmlK7dms8vE01AgMBAAGj
gb4wgbswDAYDVR0TAQH/BAIwADALBgNVHQ8EBAMCBsAwHQYDVR0OBBYEFMeUzGYV
bXwJNQVbY1+A8YXYZY8pMEIGA1UdIwQ7MDmAFJvEsUi7+D8vp8xcWvnEdVBGkpoW
oR6kHDAaMRgwFgYDVQQDDA9wb255dG93biBSU0EgQ0GCAXswOwYDVR0RBDQwMoIO
dGVzdHNlcnZlci5jb22CFXNlY29uZC50ZXN0c2VydmVyLmNvbYIJbG9jYWxob3N0
MA0GCSqGSIb3DQEBCwUAA4IBgQBsk5ivAaRAcNgjc7LEiWXFkMg703AqDDNx7kB1
RDgLalLvrjOfOp2jsDfST7N1tKLBSQ9bMw9X4Jve+j7XXRUthcwuoYTeeo+Cy0/T
1Q78ctoX74E2nB958zwmtRykGrgE/6JAJDwGcgpY9kBPycGxTlCN926uGxHsDwVs
98cL6ZXptMLTR6T2XP36dAJZuOICSqmCSbFR8knc/gjUO36rXTxhwci8iDbmEVaf
BHpgBXGU5+SQ+QM++v6bHGf4LNQC5NZ4e4xvGax8ioYu/BRsB/T3Lx+RlItz4zdU
XuxCNcm3nhQV2ZHquRdbSdoyIxV5kJXel4wCmOhWIq7A2OBKdu5fQzIAzzLi65EN
RPAKsKB4h7hGgvciZQ7dsMrlGw0DLdJ6UrFyiR5Io7dXYT/+JP91lP5xsl6Lhg9O
FgALt7GSYRm2cZdgi9pO9rRr83Br1VjQT1vHz6yoZMXSqc4A2zcN2a2ZVq//rHvc
FZygs8miAhWPzqnpmgTj1cPiU1M=
-----END CERTIFICATE-----
"#;

const KEY: &str = r#"
-----BEGIN RSA PRIVATE KEY-----
MIIEpAIBAAKCAQEAqVYYdfxTT9qr1np22UoIWq4v1E4cHncp35xxu4HNyZsoJBHR
K1gTvwh8x4LMe24lROW/LGWDRAyhaI8qDxxlitm0DPxU8p4iQoDQi3Z+oVKqsSwJ
pd3MRlu+4QFrveExwxgdahXvnhYgFJw5qG/IDWbQM0+ism/yRiXaxFNMI/kXe8FG
+JKSyJzR/yXPqM9ootgIzWxjmV50c+4eyr97DvbwAQcmHi3Ao96p4XoxzKlYWwE9
TA+s0NvmCgYxOdjLEClP8YVKbvSpFMi4dHMZId86xYioeFbr7XPp+2njr9oyZjpd
Xa9Fy5UhwZZqCqh+nQk0m3XUC5pSu3ZrPLxNNQIDAQABAoIBAFKtZJgGsK6md4vq
kyiYSufrcBLaaEQ/rkQtYCJKyC0NAlZKFLRy9oEpJbNLm4cQSkYPXn3Qunx5Jj2k
2MYz+SgIDy7f7KHgr52Ew020dzNQ52JFvBgt6NTZaqL1TKOS1fcJSSNIvouTBerK
NCSXHzfb4P+MfEVe/w1c4ilE+kH9SzdEo2jK/sRbzHIY8TX0JbmQ4SCLLayr22YG
usIxtIYcWt3MMP/G2luRnYzzBCje5MXdpAhlHLi4TB6x4h5PmBKYc57uOVNngKLd
YyrQKcszW4Nx5v0a4HG3A5EtUXNCco1+5asXOg2lYphQYVh2R+1wgu5WiDjDVu+6
EYgjFSkCgYEA0NBk6FDoxE/4L/4iJ4zIhu9BptN8Je/uS5c6wRejNC/VqQyw7SHb
hRFNrXPvq5Y+2bI/DxtdzZLKAMXOMjDjj0XEgfOIn2aveOo3uE7zf1i+njxwQhPu
uSYA9AlBZiKGr2PCYSDPnViHOspVJjxRuAgyWM1Qf+CTC0D95aj0oz8CgYEAz5n4
Cb3/WfUHxMJLljJ7PlVmlQpF5Hk3AOR9+vtqTtdxRjuxW6DH2uAHBDdC3OgppUN4
CFj55kzc2HUuiHtmPtx8mK6G+otT7Lww+nLSFL4PvZ6CYxqcio5MPnoYd+pCxrXY
JFo2W7e4FkBOxb5PF5So5plg+d0z/QiA7aFP1osCgYEAtgi1rwC5qkm8prn4tFm6
hkcVCIXc+IWNS0Bu693bXKdGr7RsmIynff1zpf4ntYGpEMaeymClCY0ppDrMYlzU
RBYiFNdlBvDRj6s/H+FTzHRk2DT/99rAhY9nzVY0OQFoQIXK8jlURGrkmI/CYy66
XqBmo5t4zcHM7kaeEBOWEKkCgYAYnO6VaRtPNQfYwhhoFFAcUc+5t+AVeHGW/4AY
M5qlAlIBu64JaQSI5KqwS0T4H+ZgG6Gti68FKPO+DhaYQ9kZdtam23pRVhd7J8y+
xMI3h1kiaBqZWVxZ6QkNFzizbui/2mtn0/JB6YQ/zxwHwcpqx0tHG8Qtm5ZAV7PB
eLCYhQKBgQDALJxU/6hMTdytEU5CLOBSMby45YD/RrfQrl2gl/vA0etPrto4RkVq
UrkDO/9W4mZORClN3knxEFSTlYi8YOboxdlynpFfhcs82wFChs+Ydp1eEsVHAqtu
T+uzn0sroycBiBfVB949LExnzGDFUkhG0i2c2InarQYLTsIyHCIDEA==
-----END RSA PRIVATE KEY-----
"#;

struct AppState {
    config: Config<>,
}

#[derive(Deserialize)]
struct PinghostRequest {
    addr: String,
}

#[poem::handler]
fn post_pinghost(_state: poem::web::Data<&Arc<AppState>>, poem::web::Form(req): poem::web::Form<PinghostRequest>) -> poem::web::Html<String>  {
    let addr = req.addr.parse().unwrap();
    let timeout = Duration::from_millis(250);

    poem::web::Html(hypertext::maud! {
        @match ping::rawsock::ping(
            addr,
            Some(timeout),
            Some(166), // ttl
            Some(3), // ident
            Some(5), // seq_cnt
            Some(&random()), // payload
        )
        {
            Ok(..) => button .{ "uk-button uk-button-small uk-button-success" } { ("Success") },
            Err(_e) => button .{ "uk-button uk-button-small uk-button-danger" } { ("Failure") }
        }
    }.render().into())
}

#[derive(Serialize)]
struct DLease {
    hostname: String
}

#[derive(Serialize)]
struct DLeases {
    leases: Vec<DLease>
}

#[poem::handler]
async fn get_leases_json(state: poem::web::Data<&Arc<AppState>>) -> poem::web::Json<DLeases> {
    let leases_file_contents = fs::read_to_string(state.config.get_string("leases.filepath").unwrap())
        .expect("Should have been able to read the file");

    let leases = dhcpd_parser::parser::parse(leases_file_contents).unwrap().leases;

    poem::web::Json(DLeases {
        leases: leases.all().iter().map(|l| DLease { hostname: l.ip.clone() }).collect::<Vec<DLease>>()
    })
}

#[allow(non_upper_case_globals)]
#[allow(dead_code)]
trait HtmxAttributes: GlobalAttributes {
    const hx_post: hypertext::Attribute = hypertext::Attribute;
    const hx_vals: hypertext::Attribute = hypertext::Attribute;
    const hx_target: hypertext::Attribute = hypertext::Attribute;
    const hx_swap: hypertext::Attribute = hypertext::Attribute;
}
impl<T: GlobalAttributes> HtmxAttributes for T {}

#[poem::handler]
async fn get_leases_html(state: poem::web::Data<&Arc<AppState>>) -> poem::web::Html<String> {
    let leases_file_contents = fs::read_to_string(state.config.get_string("leases.filepath").unwrap())
        .expect("Should have been able to read the file");

    let leases = dhcpd_parser::parser::parse(leases_file_contents).unwrap().leases;

    let mut f = timeago::Formatter::with_language(timeago_short::EnglishShort);

    poem::web::Html(hypertext::maud! {
        table .{ "uk-table uk-table-striped uk-table-small uk-text-nowrap" } {
            thead {
                tr {
                    th { ("Hostname") }
                    th { ("IP Address") }
                    th { ("Lease Age") }
                    th { }
                }
            }
            tbody {
                @for (lease, i) in leases.all().iter().zip(1..) {
                    tr #{ "leasetable-" (i) } {
                        td { (lease.client_hostname.clone()) }
                        td { (lease.ip.clone()) }
                        td {
                            (match
                                NaiveDateTime::parse_from_str(
                                    &lease.dates.starts.unwrap().to_string(),
                                    "%A %Y/%m/%d %H:%M:%S" // "Tuesday 2019/01/01 22:00:00"
                                )
                            {
                                Ok(ndt) => f.convert_chrono(ndt.and_utc(), Utc::now()),
                                Err(_e) => "Error parsing Datetime".to_string()
                            })
                        }
                        td { button .{ "uk-button uk-button-small uk-button-secondary" } hx-post="/api/pinghost" hx-vals=(format!("js:{{addr: '{}'}}", lease.ip)) hx-target="this" hx-swap="outerHTML" { ("Test Ping")} }
                    }
                }
            }
        }
    }
    .render().into())
}

#[poem::handler]
async fn get_sysinfo_html(_state: poem::web::Data<&Arc<AppState>>) -> poem::web::Html<String> {

    let sys = System::new();

    let uptime = sys.uptime().unwrap();
    let loadavg = sys.load_average().unwrap();
    let mem = sys.memory().unwrap();

    poem::web::Html(hypertext::maud! {
        table .{ "uk-table uk-table-divider uk-table-small uk-text-nowrap" } {
            tr {
                td { ("Uptime") }
                td {
                    ( format!("{:?}", uptime) )
                }
            }
            tr {
                td { ("Load") }
                td {
                    ( format!("{:.2} / {:.2} / {:.2}", loadavg.one, loadavg.five, loadavg.fifteen) )
                }
            }
            tr {
                td { ("Memory") }
                td {
                    ( format!("{} used / {} total", saturating_sub_bytes(mem.total, mem.free), mem.total) )
                }
            }
            tr {
                td { ("Network") }
                td {
                    @match sys.networks() {
                        Ok(netifs) => {
                            ul .{ "uk-list" } {
                                @for (netif, _i) in netifs.values().zip(1..) {
                                    li {
                                        span .{ "uk-inline uk-width-small" }  {
                                            (format!("{}\n", netif.name))
                                        }
                                        @match sys.network_stats(&netif.name) {
                                            Ok(netif_stats) => span { (format!("RX: {}, TX: {}", netif_stats.rx_bytes, netif_stats.tx_bytes )) }
                                            Err(_x) => span { (format!("---", )) }
                                        }
                                    }
                                }
                            }
                        }
                        Err(_x) => ("Error querying interfaces".to_string())
                    }
                }
            }
        }
    }
    .render().into())
}
   

#[tokio::main]
async fn main() -> anyhow::Result<()> {

    // Config
    let state_config = Config::builder()
        .set_default("state.database_url", "application_state.db").unwrap()
        .set_default("leases.filepath", "/var/db/dhcpd.leases").unwrap()
        .set_default("web.host", "127.0.0.1:10443").unwrap()
        .add_source(config::File::with_name(CONFIG_DIRPATH))
        .build()
        .expect("Failed to build config");

    let state = Arc::new(AppState {
        config: state_config.clone(),
    });

    let app = Route::new()
        .nest(
            "/",
            StaticFilesEndpoint::new("htdocs/"),
        )
        .at("/api/leases/json", poem::get(get_leases_json).with(poem::middleware::AddData::new(Arc::clone(&state))))
        .at("/api/leases/html", poem::get(get_leases_html).with(poem::middleware::AddData::new(Arc::clone(&state))))
        .at("/api/sysinfo/html", poem::get(get_sysinfo_html).with(poem::middleware::AddData::new(Arc::clone(&state))))
        .at("/api/pinghost", poem::post(post_pinghost).with(poem::middleware::AddData::new(Arc::clone(&state))));

    let listener = TcpListener::bind(state.config.get_string("web.host").unwrap())
        .rustls(RustlsConfig::new().fallback(RustlsCertificate::new().key(KEY).cert(CERT)));

    println!("Starting web server on https://{}/", state.config.get_string("web.host").unwrap());

    Server::new(listener).run_with_graceful_shutdown(
            app,
            async move {
                let _ = tokio::signal::ctrl_c().await;
            },
            Some(Duration::from_secs(5)),
        ).await.unwrap();

    println!("Exiting");

    Ok(())
}


