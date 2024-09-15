# RBIR [![](https://img.shields.io/discord/1283371436773212212?logo=discord&label=discord)](https://discord.gg/SshxvYpn)

`RBIR` stands for **Rewrite Bigdata in Rust**. RBIR aims to create a big data ecosystem using Rust.

This project declares our manifesto and serves as a collection of RBIR projects and posts for anyone interested in joining this journey.

## Projects

### New Projects

- [tikv](https://github.com/tikv/tikv): Distributed transactional key-value database
- [databend](https://github.com/datafuselabs/databend/): A Rust data warehouse, alternative to [Snowflake](https://www.snowflake.com/en/)
- [quickwit](https://github.com/quickwit-oss/quickwit): A Rust search engine, alternative to [Elasticsearch](https://www.elastic.co/elasticsearch)
- [risingwave](https://github.com/risingwavelabs/risingwave): Rust streaming processing engine
- [Apache DataFusion](https://github.com/apache/datafusion): Fast, extensible query engine built in rust
- [influxdb](https://github.com/influxdata/influxdb): Scalable datastore for metrics, events, and real-time analytics
- [greptimedb](https://github.com/GreptimeTeam/greptimedb): Time series database for metrics, logs and events
- [Apache HoraeDB (incubating)](https://github.com/apache/horaedb): High-performance, distributed, cloud native time-series database
- [paradedb](https://github.com/paradedb/paradedb): Postgres for Search and Analytics
- [glaredb](https://github.com/GlareDB/glaredb): An analytics DBMS for distributed data
- [fluvio](https://github.com/infinyon/fluvio): Lean and mean distributed stream processing system
- [lancedb](https://github.com/lancedb/lancedb): Developer-friendly, database for multimodal AI
- [slatedb](https://github.com/slatedb/slatedb): Cloud-native embedded storage engine built on object storage
- [daft](https://github.com/Eventual-Inc/Daft): Distributed DataFrame for Python designed for the cloud, powered by Rust
- [arroyo](https://github.com/ArroyoSystems/arroyo): Distributed stream processing engine written in Rust, designed to perform stateful computations on data

### New implementations

- [arrow-rs](https://github.com/apache/arrow-rs): Rust implementation of [Apache Arrow](https://arrow.apache.org/)
- [iceberg-rust](https://github.com/apache/iceberg-rust/): Rust implementation of [Apache Iceberg](https://iceberg.apache.org/)
- [paimon-rust](https://github.com/apache/paimon-rust): Rust implementation of [Apache Paimon](https://paimon.apache.org/)
- [hudi-rs](https://github.com/apache/hudi-rs): Rust implementation of [Apache Hudi](https://hudi.apache.org/)
- [parquet-rs](https://github.com/apache/arrow-rs/tree/master/parquet): Rust implementation of [Apache Parquet](https://parquet.apache.org/)
- [avro-rust](https://github.com/apache/avro/tree/main/lang/rust): Rust implementation of [Apache Avro](https://avro.apache.org/)
- [orc-rs](https://github.com/datafusion-contrib/datafusion-orc): Rust implementation of [Apache ORC](https://orc.apache.org/)

### New integrations

- [Apache OpenDAL](https://github.com/apache/opendal) provides python, nodejs, java, go bindings
- Apache Iceberg is now working on [building rust core for pyiceberg](https://github.com/apache/iceberg-rust/pull/518)
- Apache Paimon is going to [build paimon-py by its rust core](https://lists.apache.org/thread/q3zxcomfq441t6o8y8dslos1qvb984j0)
- [Apache DataFusion Comet](https://github.com/apache/datafusion-comet) is a high-performance accelerator for Apache Spark
- [blaze](https://github.com/kwai/blaze): The Blaze accelerator for [Apache Spark](https://spark.apache.org/) leverages native vectorized execution to accelerate query processing
- Apache Uniffle is working on it's rust shuffle server at [incubator-uniffle/rust/experimental/server](https://github.com/apache/incubator-uniffle/tree/master/rust/experimental/server)

## Blog

- [Rewrite Bigdata in Rust](https://xuanwo.io/2024/07-rewrite-bigdata-in-rust/)
