use crate::parser::asp_arguments::AspArguments;
use crate::parser::asp_subscription::AspSubscription;

#[derive(Debug)]
pub enum AspPrimarySuffix {
    Argument(AspArguments),
    Subscription(AspSubscription),
}