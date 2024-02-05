# Installation


## Install, Rust, Make and Cargo Tools 

### Rust

Make sure, you have Rust version 1.75 or higher installed. The official install 
guide is online at: https://www.rust-lang.org/tools/install. Also, ensure you have Rust nightly installed via cargo:

```bash 
    rustup toolchain install nightly
```
 Please keep Rust default on stable. Nightly is only used by some tools
that are configured via command line arguments to use nightly. To ensure your on the stable toolchain of Rust, run the following command:

```bash 
    rustup default stable
```

### Make

Make sure, you have Make installed. On Linux, you can install them with the package manager of your linux distribution. [Details in this article](https://ioflood.com/blog/install-make-command-linux/#:~:text=In%20most%20Linux%20distributions%2C%20the,command%20sudo%20yum%20install%20make%20.).  On MacOS you can use homebrew and simply [install the formula](https://formulae.brew.sh/formula/make).

### Cargo Tools

Cargo works as expected, but in addition to cargo, a makefile exists
that abstracts over several additional tools you may have to install
before all make commands work. To do so, please run the following command:

```bash 
    make setup
```

The make install command tests and tries to install all required developer dependencies.
if the automatic install fails, the script the  will show you installation instructions.

After all dependencies have been installed, the following commands are ready to use.

```
make
    make qdgw   	Start the Start the Quant Data Gateway (QDGW).
    make symdb   	Start the Symbol Master Database Service (SYMDB)
    make example     	Run the example code in flv_examples.
    make build   	Builds the code base incrementally (fast) for dev.
    make check   	Checks the code base for security vulnerabilities.
    make doc   		Builds, tests, and opens api docs in a browser.
    make fix   		Fixes linting issues as reported by clippy.
    make import   	Imports tick data from CSV into QuestDB.
    make format   	Formats call code according to cargo fmt style.
    make setup   	Tests and installs all make script dependencies.
    make run   		Runs the default binary (QDGW).
    make update   	Update rust, update and build the project.
    make test   	Tests across all crates.
    make sbe   		Generates Rust bindings from the SBE schema. 
```

The scripts called by each make command are located in the [script folder.](../scripts)

## Install Protoc

The SYMDB services relies on gRPC and uses protocol buffers to define the service interface that is implemented with Prost.
However, Prost does not ship the proto compiler anymore, therefore we need to install protoc. 

**Linux**

```bash 
   apt install -y protobuf-compiler
   
   protoc --version
```

**MacOS**

```bash
   brew install protobuf
   
   protoc --version
```
   
Other methods of installation are available on the [official protoc website](https://grpc.io/docs/protoc-installation/).


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

[doc/quest_db_config](quest_db_config)


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

## Optional: Install Autometrics

Autometrics is an observability micro-framework built for developers.
It makes it easy to instrument any function with the most useful metrics: request rate, error rate, and latency.

Install the autometrics cli:

* MacOS: brew install autometrics-dev/tap/am
* Linux ARM64: curl -L https://github.com/autometrics-dev/am/releases/latest/download/am-linux-aarch64 -o am
* Linux Intel/AMD: curl -L https://github.com/autometrics-dev/am/releases/latest/download/am-linux-x86_64 -o am

 **make autometrics executable with chmod**

```bash 
    chmod u+x am
```

On Linux, make sure the binary is in your shell path.

For more details, see the [documentation](https://docs.autometrics.dev/local-development#getting-started-with-am)
and the [github repo](https://github.com/autometrics-dev/autometrics-rs).

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

The scripts called by each make command are located in the [script folder.](../scripts)


## Optional: Install Java 17

While the entire project is written in Rust, the official SBE Tool that generates the SBE Rust bindings requires 
a Java 11 or 17 runtime. If you intend to modify the the XML SBE Schema, you need a Java installation to run the make SBE command
to re-generate the SBE Runtime. Note, if you do not intend to develop custom SBE messages, there is no need to install
Java because the latest SBE Rust bindings are stored in the repository. 
See the[ flv_sbe/bindings crate](../flv_sbe/bindings) for details. 

If you want to develop with SBE and if you want to define custom message types, you need a Java 11 or 17 runtime.
Newer Java versions may work, but have not been tested. Also, official Oracle Java distributions
as well as non-Oracle Java versions usually work equally well. For the officially (archived) Java 17 distribution, 
please visit the Oracle download page for [download and installation instructions.](https://www.oracle.com/java/technologies/javase/jdk17-archive-downloads.html) 

Usually, Linux distribution already ship with a variant of OpenJDK, so please use your package manger to install Java 17.

**Linux:**

For Debian or Ubunutu, you can usually run:
```bash 
    apt-get update
    apt-get upgrade
    
    apt install openjdk-17-jdk openjdk-17-jre
```

**Mac:** 

```bash 
    brew install openjdk 
```

This may installs a newer Java version. Please run ```make sbe``` and see if everything works as expected. 

If you see the following error:

```bash 
    make sbe
    
The operation couldn’t be completed. Unable to locate a Java Runtime.
Please visit http://www.java.com for information on installing Java.

make: *** [sbe] Error 1
```

This means that you have either no Java installed, or if you have installed it but Java is not in the default search path.

To fix this, please read the[ following post for MacOS](https://stackoverflow.com/questions/69875335/macos-how-to-install-java-17)
and this article to set the[ Java path on Linux and windows systems](https://www.geeksforgeeks.org/how-to-set-java-path-in-windows-and-linux/). 