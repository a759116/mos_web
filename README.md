# API Development with Rust
## Abstract
This article is an extension of article [Application Development with Rust](https://github.com/a759116/mos_rust#application-development-with-rust). The purpose of this article is to demonstrate development of API using Rust. This article will provide an example design and implementation of REST API to expose functionality already developed in [repo](https://github.com/a759116/mos_rust). The hope is that the reader will be able to explore the example implementation and extend this to further solidify their understanding of developing API using Rust.

## Introduction
TBW
## Use-cases
The existing [application](https://github.com/a759116/mos_rust/tree/main/src) has two modules. The module Memory has different algorithms for allocating memory for a requesting process. Here are the list of functions implemented in this module.
* best_fit_allocate
* first_fit_allocate
* worst_fit_allocate
* next_fit_allocate
* release_memory

Similarly, the module Virtual has implementation for a number of algorithms for allocating virtual memeory. This article will provide an example implementation of a REST API that is exposing best_fit_allocate functionality. The hope is that the reader will follow the example and expose other functionalities through REST APIs.

## Design
I used [actix](https://actix.rs/) framework for developing APIs in Rust. 

This web application is configured to run on locatlhost (127.0.0.1) and port 8080. This configuration has been implemented in **src/main.rs**. Few other things that have been configure in src/main.rs.
* Tag actix_web::main is exposing this application as a web application.
* HttpServer is instantiating the embedded web server to service web requests.
* There are two service function calls to configure two different paths
  * localhost:8080 : This serves the contents common across all modules. I'm calling it home and it's currently responding text "OS Services ...". 
  * localhost:8080/os/memory/ : This serves the contents from Memory mocule. <br />
  The intent for this design is to provide different contexts for different resources as per REST best practices. The hope is that the reader will extend   this repo and add a new path  localhost:8080/os/virtual/ to serve contents from Virtual module. <br />
* The path localhost:8080/os/memory/ requires a path parameter "alloc_method". The current implementation doesn't require any specif value for alloc_method, it can be anything. The hope is that the reader will modify this code to call different functions based on allocation mrthod requested. The reader may want to explore Rust enum structure and pattern matching feature "match" to implement this. 



## Run and Extend
TBW

## Conclusion
TBW
