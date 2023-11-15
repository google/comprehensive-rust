# Generating gn build rules

A tool called `gnrt` converts from `third_party.toml` into the `BUILD.gn` rules
required by our build system. It can also download crate source code.

At the moment, this is a multi-step process - it may be streamlined in future.
You'll need to mix:

* generating build files
* downloading missing crates

and possibly run through the cycle a couple of times.

To start, download the crate you want like this:

```
cd chromium/src
vpython3 tools/crates/run_gnrt.py -- download CRATE_NAME CRATE_VERSION  --shipped yes --security-critical yes
```

**IMPORTANT:** although the `gnrt` tool is part of the Chromium source code,
by running this command you will be downloading and running its dependencies
from `crates.io`. See [the earlier section][1] discussing this security
decision.

In this case, you need to specify:
* The crate name. Replace any underscores with hyphens.
* The crate version. This time, give the precise version to download rather
  than the semver compatible version.
* Whether it's shipped as part of Chromium, and whether it's security critical,
  per the normal [third party addition guidelines][0]

Once you've downloaded the crate, generate the `BUILD.gn` files like this:

```
vpython3 tools/crates/run_gnrt.py gen
```

## Transitive dependencies

At this time, there's also a good chance that you'll be informed some other
crates are missing. These are the transitive dependencies of the crate you've
asked for.

For example:

```
Missing dependency: example-transitive-dependency 1.0.83
    chromium 0.1.0 (path+file:///home/you/chromium/src/third_party/rust)
    example-crate 1.0.110 (path+file:///home/you/chromium/src/third_party/rust/example-crate/v1/crate)
```

Download each of them as well.

## Once all crates are downloaded

Once all the crates are downloaded, rerun the `gen` command given above. This should:

* Create `BUILD.gn` files
* Create `README.chromium` files


[0]: https://chromium.googlesource.com/chromium/src/+/main/docs/adding_to_third_party.md#add-a-readme_chromium
[1]: ../cargo.md