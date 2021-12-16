//! # 简单工厂模式
//! ## 优点
//! 1. 工厂类包含必要的逻辑判断，可以决定在什么时候创建哪一个产品的实例。客户端可以免除直接创建产品对象的职责
//! 2. 客户端无需知道所创建具体产品的类名，只需知道参数即可
//! 3. 也可以引入配置文件，在不修改客户端代码的情况下更换和添加新的具体产品类。
//! ## 缺点
//! 1. 工厂类集中了所有产品的创建逻辑，职责过重，一旦异常，整个系统将受影响
//! 2. 使用简单工厂模式会增加系统中类的个数(引入新的工厂类)，增加系统的复杂度和理解难度
//! 3. 系统扩展困难，一旦增加新产品不得不修改工厂逻辑，在产品类型较多时，可能造成逻辑过于复杂
//! 4. 简单工厂模式使用了static工厂方法，造成工厂角色无法形成基于继承的等级结构。
//! ## 适用环境
//! 1. 工厂类负责创建对的对象比较少，因为不会造成工厂方法中的业务逻辑过于复杂
//! 2. 客户端只知道传入工厂类的参数，对如何创建对象不关心

trait Operation {
    // 必须要 &slef ,https://doc.rust-lang.org/reference/items/traits.html 里面的 Object Safety
    fn execute(&self, a: i32, b: i32) -> i32;
}

struct OperationAdd {}
impl Operation for OperationAdd {
    fn execute(&self, a: i32, b: i32) -> i32 {
        a + b
    }
}
struct OperationSub {}
impl Operation for OperationSub {
    fn execute(&self, a: i32, b: i32) -> i32 {
        a - b
    }
}

struct OperationMul {}
impl Operation for OperationMul {
    fn execute(&self, a: i32, b: i32) -> i32 {
        a * b
    }
}

struct OperationDiv {}
impl Operation for OperationDiv {
    fn execute(&self, a: i32, b: i32) -> i32 {
        a / b
    }
}
struct OperationFactory {}
impl OperationFactory {
    fn new(oper: &str) -> Box<dyn Operation> {
        match oper {
            "+" => Box::new(OperationAdd {}),
            "-" => Box::new(OperationSub {}),
            "*" => Box::new(OperationMul {}),
            "/" => Box::new(OperationDiv {}),
            _ => Box::new(OperationAdd {}),
        }
    }
}

#[test]
fn test_simple_factory() {
    let res = OperationFactory::new("+").execute(10, 2);
    println!("{}", res);
}
