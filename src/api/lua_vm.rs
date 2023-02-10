pub trait LuaVM: super::lua_state::LuaState {
    /// 返回当前pc，测试用
    fn pc(&self) -> isize;
    /// 跳转用
    fn add_pc(&mut self, n: isize);
    /// 取出指令，同时+1
    fn fetch(&mut self) -> u32;
    /// 将常量推入栈顶
    fn get_const(&mut self, idx: isize);
    /// 将常量或栈值推入栈顶
    fn get_rk(&mut self, rk: isize);
}
