```

std = @import("std");
Print = std.io.print;

// SomeScope : Type = 
SomeScope := {
    {
        Print;
        Print; // Error, Print Alread exists.
    };
    {
        Print,
        Print; // Ok
    };
};

struct := (Fields:type, Decls:type), {

};

// struct {//Fields} {//Decls}
A := struct Fields {
        x: i32 = 0;
        y: i32 = 0;
    } Decls {

    pub B = "Hello";
    pub C = "Mang;

    D : i32 = 10;
    F : f32 = 11;
};

A.B
A.C

do_foo := (X, Y) -> {
    position : A = .{X, Y};
    Print ("A.x: {}", {position.x});
};

do_dep := (P: type) -> type {
    return struct {
        struct.Fields = {
            state: P,
        };

        std.Init = fn() -> @This() {
            self.state = Init(P);
        };
    }
};


Add = (a: i32, b: i32) -> i32 {
    a + b
}

Add = (T: type) -> (a: T, b: T) -> (T) {
    return a + b;
}

do_P = (ctx: Ctx) -> (P: type) -> (type) {
    return struct {
        state: P,

        init = (self: undefined(@This())) -> (@This()) {
            ctx.Push(self.state.init)
            return self as @This()
        }

        deinit = (self: @This()) -> (undefined(@This())) {
            ctx.Push(self.state.deinit)
            return 
        }
    }
}

Game = Seq({
    Resources,
    Window,
    Input,
    Events,
    ECS,

    World,
});

World = Seq({
    
});

App.Main = (!) -> () {
    Print = std.IO.Print;

    C = Add(1, 2);
    C += 1;

    for (0..C) |i| {
        Print("{}\n", .{i});
    }
}


Seq = (ctx: Ctx) -> (S: []type) -> type {

}

// Haskell?

Position = {
    import Vec(data);
     
    data = (x: f32, y: f32);
    init = @This().all(0);
};

Vec = (T: type) -> type {
    "Insure that all the fields of T are numerical" {
        For(type.fields(T)) |f| {
            if(!Numerical(f.type)) {
                Compiler.Error("All members of a Vec )
            }
        }
    }

    V = comptime "Get the type of  {
        info = type.info(T)

    }

    all = (v: ) -> T {

    }
}

```

## core
    - functional language
    - Types are values
    - Type Sets `{}`, creates a type
    - Enum Literal Binder `:`, creates a new type
    - Inferred Bind `:=`, sugar for `A : $ = value`
    - Types are actually a promise of a runtime value


## Program Type

Programs are types, which means they can be passed as an argument. Programs are a set of instructions that can be computed.

```
type void = ();



type fn = (
    input: type,
    output: type,
    instructions: type,
) {
    
};

// Functions
foo := fn (x: i32, out: i32, {
    out := 0;
    V : [4]i32 := {1,2,x,4};

    for (V) |v| {
        out += v;
    }
});

C := {
    x := 3;
    x += 1;
    A := foo (x) + 1
}


foo := fn x:i32, y:i32 {
    Return { x + y }
}

bar := fn x:&mut f32, y:&mut f32, ! {
    x += 1;
    y += 1;

    let sprite = Sprite {
        .pos = {x, y},
        .size = {32, 32},
        .texture = {
            .pos = {0,0},
            .size = {}
        }
    };

    render(sprite);
}
```

```

[This] = struct;

// Struct Fields
.x = field {i32; 0;};
.y = field {i32; 0;};


// init function(proc with return effect) value that returns This
.init = return This {.x = 0, .y = 0};

// move left proc with mutation effects. effects: [set This.x, get This.x, i32.add, set This.y, get This.y, i32.Overflow]
.move_left = proc (self: mut This) {
    self.x += 1;
    self.y += 1;
}

// can be also written like
.move_left = seq {
    This.x += 1;
    This.y += 1;
}


[Main] = seq {
    catch i32.Overflow {system.panic};


    This;
    This.move_left;
    system.println "x: {}, y: {}" {This.x, This.y};
    This.move_left;
    system.println "x: {}, y: {}" {This.x, This.y};
};




export VecFns(This);

pub add = fn(this: This, other: This) This {
    return .{
        .x = this.x + other.x,
        .y = this.y + other.y,
    };
} 

```