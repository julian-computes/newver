newver
------

`newver` is a simple tool for quickly checking for updates to Maven packages. It is written in Rust and uses the Maven Central API to check for updates.

While the Maven `versions` plugin accomplishes the same task, I wanted a tool that parses a text file with the following format:

    groupId1:artifactId1
    groupId2:artifactId2

and outputs the latest version of each artifact.

## Usage

```
Usage: newver [OPTIONS]

Options:
  -a, --artifacts-file <ARTIFACTS_FILE>  [default: ~/.config/newver/artifacts]
  -i, --ignore-before <IGNORE_BEFORE>    [default: 2w]
  -h, --help                             Print help
  -V, --version                          Print version
```

```shell
# Find the newest versions of artifacts released which have had updates in the past 3 weeks
$ newver -a ./data/artifacts -i 3w
org.eclipse.jetty:jetty-server version 11.0.15 (4-11-2023)
org.xerial:sqlite-jdbc version 3.41.2.1 (3-24-2023)
org.apache.shiro:shiro-core version 2.0.0-alpha-1 (2-28-2023)
org.glassfish.jersey.core:jersey-client version 3.0.10 (3-30-2023)
org.springframework:spring-context version 5.2.24.RELEASE (4-13-2023)
```
