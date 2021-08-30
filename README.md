# API Development with Rust
## Abstract
This article is an extension of article [Application Development with Rust](https://github.com/a759116/mos_rust#application-development-with-rust). The purpose of this article is to demonstrate development of API using Rust. This article will provide an example design and implementation of REST API to expose functionality already developed. The hope is that the reader will be able to explore the example implementation and extend this to further solidify their understanding of developing API using Rust.

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
  * localhost:8080/os/memory/ : This serves the contents from Memory module. <br />
  The intent for this design is to provide different contexts for different resources as per REST best practices. The hope is that the reader will extend   this repo and add a new path  localhost:8080/os/virtual/ to serve contents from Virtual module. <br />
* The path localhost:8080/os/memory/ requires a path parameter "alloc_method". The current implementation doesn't require any specific value for alloc_method, it can be anything. The hope is that the reader will modify this code to call different functions based on allocation method requested. The reader may want to explore Rust enum structure and pattern matching feature "match" to implement this. 

  | alloc_method | called function |
  | ------------ | --------------- |
  | best_fit | best_fit_allocate |
  | first_fit | first_fit_allocate |
  | worst_fit | worst_fit_allocate |
  | next_fit | next_fit_allocate |
  | release | release_memory |

The "src" folder contains code for creating different API endpoints and "tests" folder contains code to test those respective ebdpoints.

The folder "src" has a subfolder "services" that contains code for different contexts such as home, memory, and virtual. To achieve this structure in Rust,
* module services is defined in lib.rs
* different contexts are defined as modules in services.rs. As of this writing, the modules defined are home, memory, and virtual.
  * After this step, the subfolder services is created and Rust code is written for different modules in files home.rs, memory.rs, and virtual.rs. 

As of this writing, the code is there only in memory.rs to expose best_fit_allocate functionality as a REST API.
* The API is expecting a JSON payload as input. Here is an example payload <br />
  ```
  {
      "memory_map": {
          "memory_blocks": [
              {
                  "start_address": 0,
                  "end_address": 1023,
                  "segment_size": 1024,
                  "process_id": 0
              }
          ]
      },
      "req_size": 10,
      "req_process_id": 32
  }
  ```
  - To achieve this in Rust,
    - Two stucts have been defined: MemoryMap, and MemoryAllocationRequest
    - I used [Serde](https://serde.rs/) framework for serializing and deserializing these data structures. The use of derive macro from Serde generates implementations of the Serialize and Deserialize traits for these data structure.
    - The [Json helper](https://docs.rs/actix-web/3.3.2/actix_web/web/struct.Json.html) from module web in crate actix-web is used to extract MemoryAllocationRequest struct structure from input Json payload.

* The API calls function best_fit_allocate defined in module memory::memory of crate mos_rust to allocate memory.
  - To achieve this in Rust,
    - I've defined a dependency on mos_rust, and I've the [project mos_rust](https://github.com/a759116/mos_rust) at path ../mos_rust with respect to this project

* The API response is in Json format.
  - To achieve this, I used the method "json" of "HttpResponseBuilder". The "Ok" method of HttpResponse returns "HttpResponseBuilder". This helped to create the response in Json format.

With the above explanation, the hope is that the reader can follow this example and expose other functions as REST APIs.

## Run and Extend
There are two options to run and test the existing code.
1. Clone or download this code. Then go to the code directory, and run the command "cargo test". Upon successful run, you will see message like 
```
Running tests/test_memory.rs
running 2 test
test tests::test_index_get ... ok
test tests::test_best_fit_allocate_get ... ok".
```
2. Use a tool like Postman to test your API. Clone or download this code. Then go to the code directory, and run the command "cargo run". You will see message like
```
    Finished dev [unoptimized + debuginfo] target(s) in 0.26s
     Running `target/debug/mos_web
```
Now use Postman to send a POST request to  localhost:8080/os/memory/best_fit. Here is an example request and response body for the request
```
Request Body:
{
    "memory_map": {
        "memory_blocks": [
            {
                "start_address": 0,
                "end_address": 1023,
                "segment_size": 1024,
                "process_id": 0
            }
        ]
    },
    "req_size": 10,
    "req_process_id": 32
}

Response Body:
{
    "memory_blocks": [
        {
            "start_address": 0,
            "end_address": 9,
            "segment_size": 10,
            "process_id": 32
        },
        {
            "start_address": 10,
            "end_address": 1023,
            "segment_size": 1014,
            "process_id": 0
        }
    ]
}
```
The reader can expose other functions implemented in the modules Memory and Virtual as REST APIs.

## Conclusion
This was an attempt to provide an example design and implementation of REST API using Rust to help the reader to learn Rust. The reader was expected to explore the code on their own. This would be revised based on feedback from readers to achieve the goal of making it easier to learn Rust. <br />

If the reader wishes, they can build a UI using [Rust WebAssembly] (https://www.rust-lang.org/what/wasm). This will serve as a frontend to APIs.
