// Source: https://github.com/serverlesstechnology/cqrs/blob/master/tests/lib.rs

#[tokio::test]
async fn test_mem_store() {
    let event_store = MemStore::<TestAggregate>::default();
    let id = "test_id_A";
    let initial_events = event_store.load_events(&id).await.unwrap();
    assert_eq!(0, initial_events.len());
    let agg_context = event_store.load_aggregate(&id).await.unwrap();

    event_store
        .commit(
            vec![TestEvent::Created(Created {
                id: "test_event_A".to_string(),
            })],
            agg_context,
            metadata(),
        )
        .await
        .unwrap();
    let stored_events = event_store.load_events(&id).await.unwrap();
    assert_eq!(1, stored_events.len());
    let agg_context = event_store.load_aggregate(&id).await.unwrap();

    event_store
        .commit(
            vec![
                TestEvent::Tested(Tested {
                    test_name: "test A".to_string(),
                }),
                TestEvent::Tested(Tested {
                    test_name: "test B".to_string(),
                }),
                TestEvent::SomethingElse(SomethingElse {
                    description: "something else happening here".to_string(),
                }),
            ],
            agg_context,
            metadata(),
        )
        .await
        .unwrap();
    let stored_envelopes = event_store.load_events(&id).await.unwrap();

    let mut agg = TestAggregate::default();
    for stored_envelope in stored_envelopes {
        let event = stored_envelope.payload;
        agg.apply(event);
    }
    println!("{:#?}", agg);
}
