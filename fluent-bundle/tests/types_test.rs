use fluent_bundle::resolve::Scope;
use fluent_bundle::FluentBundle;
use fluent_bundle::FluentResource;
use fluent_bundle::FluentValue;
use unic_langid::langid;

#[test]
fn fluent_value_try_number() {
    let value = FluentValue::try_number("invalid");
    assert_eq!(value, "invalid".into());
}

#[test]
fn fluent_value_matches() {
    // We'll use `ars` locale since it happens to have all
    // plural rules categories.
    let langid_ars = langid!("ars");
    let bundle: FluentBundle<FluentResource> = FluentBundle::new(&[langid_ars]);
    let scope = Scope::new(&bundle, None);

    let string_val = FluentValue::from("string1");
    let string_val_copy = FluentValue::from("string1");
    let string_val2 = FluentValue::from("23.5");

    let number_val = FluentValue::from(-23.5);
    let number_val_copy = FluentValue::from(-23.5);
    let number_val2 = FluentValue::from(23.5);

    assert_eq!(string_val.matches(&string_val_copy, &scope), true);
    assert_eq!(string_val.matches(&string_val2, &scope), false);

    assert_eq!(number_val.matches(&number_val_copy, &scope), true);
    assert_eq!(number_val.matches(&number_val2, &scope), false);

    assert_eq!(string_val2.matches(&number_val2, &scope), false);

    assert_eq!(string_val2.matches(&number_val2, &scope), false);

    let string_cat_zero = FluentValue::from("zero");
    let string_cat_one = FluentValue::from("one");
    let string_cat_two = FluentValue::from("two");
    let string_cat_few = FluentValue::from("few");
    let string_cat_many = FluentValue::from("many");
    let string_cat_other = FluentValue::from("other");

    let number_cat_zero = 0.into();
    let number_cat_one = 1.into();
    let number_cat_two = 2.into();
    let number_cat_few = 3.into();
    let number_cat_many = 11.into();
    let number_cat_other = 101.into();

    assert_eq!(string_cat_zero.matches(&number_cat_zero, &scope), true);
    assert_eq!(string_cat_one.matches(&number_cat_one, &scope), true);
    assert_eq!(string_cat_two.matches(&number_cat_two, &scope), true);
    assert_eq!(string_cat_few.matches(&number_cat_few, &scope), true);
    assert_eq!(string_cat_many.matches(&number_cat_many, &scope), true);
    assert_eq!(string_cat_other.matches(&number_cat_other, &scope), true);
    assert_eq!(string_cat_other.matches(&number_cat_one, &scope), false);

    assert_eq!(string_val2.matches(&number_cat_one, &scope), false);
}

#[test]
fn fluent_value_from() {
    let value_str = FluentValue::from("my str");
    let value_string = FluentValue::from(String::from("my string"));
    let value_f64 = FluentValue::from(23.5);
    let value_isize = FluentValue::from(-23);

    assert_eq!(value_str, "my str".into());
    assert_eq!(value_string, "my string".into());

    assert_eq!(value_f64, FluentValue::from(23.5));
    assert_eq!(value_isize, FluentValue::from(-23));
}
