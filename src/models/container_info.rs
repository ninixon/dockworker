use models::{Config, Mount, NetworkSettings, State};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ContainerInfo {
  pub app_armor_profile: String,
  pub args: Vec<String>,
  pub config: Config,
  pub created: String,
  pub driver: String,
  // ExecIDs
  // GraphDriver
  // HostConfig
  pub hostname_path: String,
  pub hosts_path: String,
  pub id: String,
  pub image: String,
  pub log_path: String,
  pub mount_label: String,
  pub mounts: Vec<Mount>,
  pub name: String,
  pub network_settings: NetworkSettings,
  pub path: String,
  pub process_label: String,
  pub resolv_conf_path: String,
  pub restart_count: u64,
  pub state: State,
}

impl std::fmt::Display for ContainerInfo {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::result::Result<(), std::fmt::Error> {
    write!(f, "{}", self.id)
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  use serde_json;

  #[test]
  fn deserialize() {
    serde_json::from_str::<ContainerInfo>(&r#"
      {
        "Id": "774758ca1db8d05bd848d2b3456c8253a417a0511329692869df1cbe82978d37",
        "Created": "2016-10-25T11:59:37.858589354Z",
        "Path": "rails",
        "Args": ["server", "-b", "0.0.0.0"],
        "State": {
          "Status": "running",
          "Running": true,
          "Paused": false,
          "Restarting": false,
          "OOMKilled": false,
          "Dead": false,
          "Pid": 13038,
          "ExitCode": 0,
          "Error": "",
          "StartedAt": "2016-10-25T11:59:38.261828009Z",
          "FinishedAt": "0001-01-01T00:00:00Z"
        },
        "Image": "sha256:f5e9d349e7e5c0f6de798d732d83fa5e087695cd100149121f01c891e6167c13",
        "ResolvConfPath": "/var/lib/docker/containers/774758ca1db8d05bd848d2b3456c8253a417a0511329692869df1cbe82978d37/resolv.conf",
        "HostnamePath": "/var/lib/docker/containers/774758ca1db8d05bd848d2b3456c8253a417a0511329692869df1cbe82978d37/hostname",
        "HostsPath": "/var/lib/docker/containers/774758ca1db8d05bd848d2b3456c8253a417a0511329692869df1cbe82978d37/hosts",
        "LogPath": "/var/lib/docker/containers/774758ca1db8d05bd848d2b3456c8253a417a0511329692869df1cbe82978d37/774758ca1db8d05bd848d2b3456c8253a417a0511329692869df1cbe82978d37-json.log",
        "Name": "/railshello_web_1",
        "RestartCount": 0,
        "Driver": "aufs",
        "MountLabel": "",
        "ProcessLabel": "",
        "AppArmorProfile": "",
        "ExecIDs": null,
        "HostConfig": {
          "Binds": [],
          "ContainerIDFile": "",
          "LogConfig": {
            "Type": "json-file",
            "Config": {}
          },
          "NetworkMode": "railshello_default",
          "PortBindings": {
            "3000/tcp": [
              {
                "HostIp": "",
                "HostPort": "3000"
              }
            ]
          },
          "RestartPolicy": {
            "Name": "",
            "MaximumRetryCount": 0
          },
          "AutoRemove": false,
          "VolumeDriver": "",
          "VolumesFrom": [],
          "CapAdd": null,
          "CapDrop": null,
          "Dns": null,
          "DnsOptions": null,
          "DnsSearch": null,
          "ExtraHosts": null,
          "GroupAdd": null,
          "IpcMode": "",
          "Cgroup": "",
          "Links": null,
          "OomScoreAdj": 0,
          "PidMode": "",
          "Privileged": false,
          "PublishAllPorts": false,
          "ReadonlyRootfs": false,
          "SecurityOpt": null,
          "UTSMode": "",
          "UsernsMode": "",
          "ShmSize": 67108864,
          "Runtime": "runc",
          "ConsoleSize": [0, 0],
          "Isolation": "",
          "CpuShares": 0,
          "Memory": 0,
          "CgroupParent": "",
          "BlkioWeight": 0,
          "BlkioWeightDevice": null,
          "BlkioDeviceReadBps": null,
          "BlkioDeviceWriteBps": null,
          "BlkioDeviceReadIOps": null,
          "BlkioDeviceWriteIOps": null,
          "CpuPeriod": 0,
          "CpuQuota": 0,
          "CpusetCpus": "",
          "CpusetMems": "",
          "Devices": null,
          "DiskQuota": 0,
          "KernelMemory": 0,
          "MemoryReservation": 0,
          "MemorySwap": 0,
          "MemorySwappiness": -1,
          "OomKillDisable": false,
          "PidsLimit": 0,
          "Ulimits": null,
          "CpuCount": 0,
          "CpuPercent": 0,
          "IOMaximumIOps": 0,
          "IOMaximumBandwidth": 0
        },
        "GraphDriver": {
          "Name": "aufs",
          "Data": null
        },
        "Mounts": [],
        "Config": {
          "Hostname": "774758ca1db8",
          "Domainname": "",
          "User": "",
          "AttachStdin": false,
          "AttachStdout": false,
          "AttachStderr": false,
          "ExposedPorts": {
            "3000/tcp": {}
          },
          "Tty": false,
          "OpenStdin": false,
          "StdinOnce": false,
          "Env": [
            "RACK_ENV=development",
            "PROJECT_NAME=rails_hello",
            "GLOBAL_PASSWORD=magic",
            "SOME_PASSWORD=secret",
            "RAILS_ENV=development",
            "DATABASE_URL=postgres://postgres@db:5432/rails_hello_development",
            "PATH=/usr/local/bundle/bin:/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin",
            "RUBY_MAJOR=2.3",
            "RUBY_VERSION=2.3.1",
            "RUBY_DOWNLOAD_SHA256=b87c738cb2032bf4920fef8e3864dc5cf8eae9d89d8d523ce0236945c5797dcd",
            "RUBYGEMS_VERSION=2.6.7",
            "BUNDLER_VERSION=1.13.4",
            "GEM_HOME=/usr/local/bundle",
            "BUNDLE_PATH=/usr/local/bundle",
            "BUNDLE_BIN=/usr/local/bundle/bin",
            "BUNDLE_SILENCE_ROOT_WARNING=1",
            "BUNDLE_APP_CONFIG=/usr/local/bundle"
          ],
          "Cmd": ["rails", "server", "-b", "0.0.0.0"],
          "Image": "faraday/rails_hello",
          "Volumes": null,
          "WorkingDir": "/usr/src/app",
          "Entrypoint": null,
          "OnBuild": null,
          "Labels": {
            "com.docker.compose.config-hash": "ff040c76ba24b1bac8d89e95cfb5ba7e29bd19423ed548a1436ae3c94bc6381a",
            "com.docker.compose.container-number": "1",
            "com.docker.compose.oneoff": "False",
            "com.docker.compose.project": "railshello",
            "com.docker.compose.service": "web",
            "com.docker.compose.version": "1.8.1",
            "io.fdy.cage.lib.coffee_rails": "/usr/src/app/vendor/coffee-rails",
            "io.fdy.cage.pod": "frontend",
            "io.fdy.cage.shell": "bash",
            "io.fdy.cage.srcdir": "/usr/src/app",
            "io.fdy.cage.target": "development",
            "io.fdy.cage.test": "bundle exec rake"
          }
        },
        "NetworkSettings": {
          "Bridge": "",
          "SandboxID": "ca243185e052f364f6f9e4141ee985397cda9c66a87258f8a8048a05452738cf",
          "HairpinMode": false,
          "LinkLocalIPv6Address": "",
          "LinkLocalIPv6PrefixLen": 0,
          "Ports": {
            "3000/tcp": [
              {
                "HostIp": "0.0.0.0",
                "HostPort": "3000"
              }
            ]
          },
          "SandboxKey": "/var/run/docker/netns/ca243185e052",
          "SecondaryIPAddresses": null,
          "SecondaryIPv6Addresses": null,
          "EndpointID": "",
          "Gateway": "",
          "GlobalIPv6Address": "",
          "GlobalIPv6PrefixLen": 0,
          "IPAddress": "",
          "IPPrefixLen": 0,
          "IPv6Gateway": "",
          "MacAddress": "",
          "Networks": {
            "railshello_default": {
              "IPAMConfig": null,
              "Links": null,
              "Aliases": ["web", "774758ca1db8"],
              "NetworkID": "4b237b1de0928a11bb399adaa249705b666bdc5dece3e9bdc260a630643bf945",
              "EndpointID": "7d5e1e9df4bdf400654b96afdd1d42040c150a4f5b414f084c8fd5c95a9a906e",
              "Gateway": "172.24.0.1",
              "IPAddress": "172.24.0.3",
              "IPPrefixLen": 16,
              "IPv6Gateway": "",
              "GlobalIPv6Address": "",
              "GlobalIPv6PrefixLen": 0,
              "MacAddress": "02:42:ac:18:00:03"
            }
          }
        }
      }
    "#).unwrap();
  }
}