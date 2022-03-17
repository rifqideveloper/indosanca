(module
(import"import""woker_in"(func $woker_in(param i32 i32)))
(import"import""mem"(memory 1))
(data(i32.const 0)"./sw.jstestingfn")
(import"import""log"(func $log(param i32)))
(func(export"main")
i32.const 0
i32.const 7
call $woker_in
i32.const 5
call $log
))
