struct AdminUser {
    username: String,
    password: String,
}

struct User {
    username: String,
    password: String,
}

trait CanEdit {
    fn edit(&self) {
        println!("Admin is editing")
    }
}

trait CanCreate {
    fn create(&self) {
        println!("Admin is creating")
    }
}

trait CanDelete {
    fn delete(&self) {
        println!("Admin is deleting")
    }
}

// these 3 implement the standard traits for Admin user as defined
impl CanDelete for AdminUser {}
impl CanCreate for AdminUser {}
impl CanEdit for AdminUser {}

// this will allow us to override and edit the username
impl CanEdit for User {
    fn edit(&self) {
        println!("A standard user {} is editing", self.username)
    }
}
// implementing the traits"

fn create<T: CanCreate>(user: &T) -> () {
    user.create();
}

fn edit<T: CanEdit>(user: &T) -> () {
    user.edit()
}

fn delete<T: CanDelete>(user: &T) -> () {
    user.delete();
}

pub fn run_traits() {
    let admin = AdminUser {
        username: "admin".to_string(),
        password: "password".to_string(),
    };

    let user = User {
        username: "user".to_string(),
        password: "password".to_string(),
    };

    create(&admin); // output Admin is creating
    edit(&admin); // output Admin is editing
    edit(&user); // output A standard user user is editing
    delete(&admin); // output Admin is deleting
}
