local t = {"a","b","c"}
t[2] = "B"
t["foo"] = "Bar"
local s = t[3]..t[2]..t[1]..t["foo"]..#t

--[[

main <.\testlua.lua:0,0> (14 instructions at 00000000001a9130)
0+ params, 6 slots, 1 upvalue, 2 locals, 9 constants, 0 functions
        1       [1]     NEWTABLE        0 3 0
        2       [1]     LOADK           1 -1    ; "a"
        3       [1]     LOADK           2 -2    ; "b"
        4       [1]     LOADK           3 -3    ; "c"
        5       [1]     SETLIST         0 3 1   ; 1
        6       [2]     SETTABLE        0 -4 -5 ; 2 "B"
        7       [3]     SETTABLE        0 -6 -7 ; "foo" "Bar"
        8       [4]     GETTABLE        1 0 -8  ; 3
        9       [4]     GETTABLE        2 0 -4  ; 2
        10      [4]     GETTABLE        3 0 -9  ; 1
        11      [4]     GETTABLE        4 0 -6  ; "foo"
        12      [4]     LEN             5 0
        13      [4]     CONCAT          1 1 5
        14      [4]     RETURN          0 1



]]