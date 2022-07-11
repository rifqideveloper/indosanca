(module
    (func $test (result i32)
        i32.const 0
        i32.const 9
        i32.store8
        i32.const 1
        (i32.add (
            i32.const 0
            i32.load8_u
        )(
            i32.const 1
        ))
        i32.store8
        i32.const 0
        i32.load8_u
    )
    (func $pop
        (local $s i32 )
        i32.const 0
        i32.const 45
        i32.store8
        i32.const 0
        i32.ne
        if 
            i32.const 1
            i32.const 2
            i32.store8
            
        end        
        loop
            
            local.get 
        end
    )
)