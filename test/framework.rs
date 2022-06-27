// Source: https://github.com/serverlesstechnology/cqrs/blob/master/tests/lib.rs

#[tokio::test]
async fn framework_test() {
    let event_store = MemStore::default();
    let stored_events = event_store.get_events();

    let delivered_events = Default::default();
    let view = TestView::new(Arc::clone(&delivered_events));

    let cqrs = CqrsFramework::new(event_store, vec![Box::new(view)], TestService);
    let uuid = uuid::Uuid::new_v4().to_string();
    let id = uuid.clone();
    let metadata = metadata();
    cqrs.execute_with_metadata(
        &id,
        TestCommand::ConfirmTest(ConfirmTest {
            test_name: uuid.clone(),
        }),
        metadata,
    )
    .await
    .unwrap_or_default();

    assert_eq!(1, stored_events.read().unwrap().len());
    assert_eq!(1, delivered_events.read().unwrap().len());

    let test = "TEST_A";
    let id = uuid.clone();
    cqrs.execute(
        &id,
        TestCommand::ConfirmTest(ConfirmTest {
            test_name: test.to_string(),
        }),
    )
    .await
    .unwrap_or_default();

    assert_eq!(2, delivered_events.read().unwrap().len());
    let stored_event_count = stored_events
        .read()
        .unwrap()
        .get(uuid.clone().as_str())
        .unwrap()
        .len();
    assert_eq!(2, stored_event_count);

    let id = uuid.clone();
    let err = cqrs
        .execute(
            &id,
            TestCommand::ConfirmTest(ConfirmTest {
                test_name: test.to_string(),
            }),
        )
        .await
        .unwrap_err();
    match err {
        AggregateError::UserError(payload) => {
            assert_eq!("test already performed", payload.0.as_str())
        }
        _ => panic!("not the expected error"),
    };

    assert_eq!(2, delivered_events.read().unwrap().len());
    let stored_event_count = stored_events
        .read()
        .unwrap()
        .get(uuid.clone().as_str())
        .unwrap()
        .len();
    assert_eq!(2, stored_event_count);
}
