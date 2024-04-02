use paper_1::utils;

#[test]
fn test() {
  assert_eq!(
    utils::string_reverse("Ciao!"),
    "!oaiC"
  );

  assert_eq!(
    utils::is_armstrong(153),
    true,
  );

  assert_eq!(
    utils::is_armstrong(154),
    false,
  );
}