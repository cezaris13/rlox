#[cfg(test)]
mod tests {
    use crate::environment::Environment;
    use crate::expression::LiteralValue::*;

    use std::rc::Rc;

    #[test]
    fn define_add_value_value_is_saved() {
        let mut environment = Environment::new();
        environment.define(String::from("test"), Nil);

        let variable = environment.get("test");

        assert!(variable.is_ok());

        assert_eq!(variable.unwrap(), Nil);
    }

    #[test]
    fn get_value_no_saved_gets_error() {
        let environment = Environment::new();

        let variable = environment.get("test");

        assert!(variable.is_err());
        assert_eq!(variable.err(), Some("Undefined variable test".to_string()));
    }

    #[test]
    fn get_value_from_enclosing() {
        let mut environment = Environment::new();
        let mut environment_enclosing = Environment::new();
        environment_enclosing.define(String::from("test"), True);

        environment.enclosing = Some(Rc::new(environment_enclosing));

        let variable = environment.get("test");

        assert!(variable.is_ok());

        assert_eq!(variable.unwrap(), True);
    }

    // #[test]
    // fn assign_value_from_enclosing() {
    //     let mut environment = Environment::new();
    //     let mut environment_enclosing = Environment::new();
    //     environment_enclosing.define(String::from("test"), True);

    //     environment.enclosing = Some(Rc::new(environment_enclosing));

    //     let result = environment.assign(String::from("test"), False);

    //     assert!(result.is_ok());

    //     assert_eq!(environment.get("test"), Ok(True));
    // }

    #[test]
    fn assign_value_does_not_exist_returns_error() {
        let mut environment = Environment::new();

        let result = environment.assign(String::from("test"), False);

        assert!(result.is_err());

        assert_eq!(
            result.err(),
            Some(String::from("Variable does not exist test"))
        );
    }

    #[test]
    fn assign_to_created_value_returns_ok() {
        let mut environment = Environment::new();

        environment.define(String::from("test"), Nil);
        let result = environment.assign(String::from("test"), False);

        assert!(result.is_ok());

        assert_eq!(environment.get("test"), Ok(False));
    }
}
