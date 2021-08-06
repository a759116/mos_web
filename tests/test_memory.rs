#[cfg(test)]

mod tests {
    use actix_web::{
        http::{Method, StatusCode},
        test, web, App,
    };
    use mos_web::services::{
        home::index,
        memory::{best_fit_allocate, MemoryAllocationRequest, MemoryBlock, MemoryMap},
    };

    #[actix_rt::test]
    async fn test_index_get() {
        let mut app = test::init_service(App::new().route("/", web::get().to(index))).await;
        let req = test::TestRequest::with_header("content-type", "text/plain").to_request();
        let resp = test::call_service(&mut app, req).await;
        assert!(resp.status().is_success());
    }

    #[actix_rt::test]
    async fn test_best_fit_allocate_get() {
        // set-up test data
        let mut mar: MemoryAllocationRequest = MemoryAllocationRequest::default();
        let mb = MemoryBlock {
            start_address: 0,
            end_address: 1023,
            segment_size: 1024,
            process_id: 0,
        };
        mar.memory_map.memory_blocks.push(mb);
        mar.req_size = 10;
        mar.req_process_id = 32;

        //call api
        let mut app = test::init_service(
            App::new().service(
                web::scope("/os")
                    .service(web::resource("/memory/{alloc_method}").route(web::post().to(best_fit_allocate))),
            ),
        ).await;

        let resp = test::TestRequest::with_header("content-type", "application/json")
            .uri("/os/memory/best_fit")
            .method(Method::POST)
            .set_json(&mar)
            .send_request(&mut app)
            .await;

        // validate
        let stat = resp.status();
        let result: MemoryMap = test::read_body_json(resp).await;

        assert_eq!(stat, StatusCode::OK);
        assert_eq!(result.memory_blocks.len(), 2);
        assert!(if_equal(&result.memory_blocks[0], 0, 9, 10, 32));
        assert!(if_equal(&result.memory_blocks[1], 10, 1023, 1014, 0));
    }

    fn if_equal(
        mb: &MemoryBlock,
        start_address: i32,
        end_address: i32,
        segment_size: i32,
        process_id: i32,
    ) -> bool {
        if mb.start_address == start_address
            && mb.end_address == end_address
            && mb.segment_size == segment_size
            && mb.process_id == process_id
        {
            return true;
        } else {
            return false;
        }
    }
}
