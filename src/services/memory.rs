pub use mos_rust::memory::{self, memory::MemoryBlock};

use actix_web::web;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct MemoryMap {
    pub memory_blocks: Vec<MemoryBlock>,
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct MemoryAllocationRequest {
    pub memory_map: MemoryMap,
    pub req_size: i32,
    pub req_process_id: i32,
}

use actix_web::{HttpResponse, Responder};

pub async fn best_fit_allocate(
    mar: web::Json<MemoryAllocationRequest>,
    web::Path(alloc_method): web::Path<String>,
) -> impl Responder {
    println!("mar: {:?}", alloc_method);
    let mut memory_blocks = mar.memory_map.memory_blocks.clone();
    let request_size = mar.req_size;
    let process_id = mar.req_process_id;

    let _mb_allocated: MemoryBlock =
        memory::memory::best_fit_allocate(&mut memory_blocks, request_size, process_id);

    let mut memory_map = MemoryMap::default();
    memory_map.memory_blocks = memory_blocks;
    HttpResponse::Ok().json(memory_map)
}
