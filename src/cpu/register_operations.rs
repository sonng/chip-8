use super::CPU;

impl CPU {

    pub(super) fn copy(&mut self, x: usize, y: usize) {
        self.registers[x] = self.registers[y];
    }
}
