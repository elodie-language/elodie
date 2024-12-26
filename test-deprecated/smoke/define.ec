type Person (name: String)

define Person {
    function say_name(){
        rt::io::println('Hi, I am ${self.name}')
    }
}

let elodie = Person(name = 'Elodie')
elodie.say_name()

// out: Hi, I am Elodie
