# Installation

## Install QuestDB

QuestDB documentation: https://questdb.io/docs/

**Docker**

```bash 
docker run -p 9000:9000 -p 9009:9009 -p 8812:8812 -p 9003:9003 questdb/questdb:7.3.7
```

**Linux**

``` 
1) Download binary: https://questdb.io/download/
2) Unpack: tar -xvf questdb-7.3.7-rt-linux-amd64.tar.gz
3) Start: ./questdb.sh start
```

**MacOS**

```bash 
brew install questdb
```

## Configure QuestDB

Unfortunately, some of QuestDB ports conflict with Fluvio ports.
Therefore, we have to change the ports in QuestDB's configuration file.

The default configuration folder:
* Linux: $HOME/.questdb/conf
* Mac (M1/M2/M3): /opt/homebrew/var/questdb/conf
* Windows: C:\Windows\System32\qdbroot/conf

In line 23 of the server config, the default IP address and port of the
http server needs to change, as shown below:

``` 
http.net.bind.to=0.0.0.0:7777
```

And in line 143, the min.net needs to be set as following:

``` 
http.min.net.bind.to=0.0.0.0:9007
```

You find a correctly re-configured sever.conf in the following folder:

[doc/quest_db_config](doc/quest_db_config)


You can copy this server.conf over the default server.conf and restart quest db
by typing in a terminal: 

``` 
 questdb stop
 
 questdb start
```

Verify that quest db is running by typing in a terminal:

```bash
questdb status
```

Which should show:

```
questdb status

  ___                  _   ____  ____
 / _ \ _   _  ___  ___| |_|  _ \| __ )
| | | | | | |/ _ \/ __| __| | | |  _ \
| |_| | |_| |  __/\__ \ |_| |_| | |_) |
 \__\_\\__,_|\___||___/\__|____/|____/
                        www.questdb.io

PID: 10339
```

The PID will be different on your machine.

Then test the http server by opening the web console in a browser on  port 7777.

http://localhost:7777/


## Install Fluvio

Fluvio documentation: https://www.fluvio.io/docs/

**Linux / Mac:** 

```bash 
   curl -fsS https://hub.infinyon.cloud/install/install.sh | bash
```

fvm will be installed at ~/.fvm/bin, and will install fluvio and the rest of the development tools at ~/.fluvio/bin. You will need to add these directories to your shell’s PATH environment variable.

```bash 
 echo 'export PATH="${HOME}/.fvm/bin:${HOME}/.fluvio/bin:${PATH}"' >> ~/.bashrc
```

## Start Fluvio


```bash 
    fluvio cluster start
```

Verify that fluvio is running by typing in a terminal:

```bash
     fluvio cluster status
```

Which should show:

```    
 -  📝 Running cluster status checks with profile local
    ✅ SC is ok
    ✅ (1/1) SPUs are online
    ✅ 0 topics using 0 B
```

## 🛠️ Cargo & Make

Cargo works as expected, but in addition to cargo, a makefile exists
that abstracts over several additional tools you may have to install
before all make commands work. To do so, please run the following command:

```bash 
    make setup
```

The make install command tests and tries to install all required developer dependencies.
if the automatic install fails, the script will show a link with further installation instructions.

After all dependencies have been installed, the following commands are ready to use.

```
    make build   	Builds the code base incrementally (fast) for dev.
    make check   	Checks the code base for security vulnerabilities.
    make fix   		Fixes linting issues as reported by clippy.
    make import   	Imports tick data from CSV into QuestDB.
    make format   	Formats call code according to cargo fmt style.
    make setup   	Tests and installs all make script dependencies.
    make run   		Runs the binary defined in scripts/run.sh.
    make update   	Update rust, pull latest from git remote, and build the project.
    make test   	Tests across all crates.
    make sbe   		Generates Rust bindings for SBE messages define in spec/sbe.
```

The scripts called by each make command are located in the [script folder.](scripts)
