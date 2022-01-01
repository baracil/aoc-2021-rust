use crate::AOCResult;
use crate::days::day_22::axis::Axis;
use crate::days::day_22::command::Command;
use crate::days::day_22::cube::Cube;

pub struct System {
    x_axis:Axis,
    y_axis:Axis,
    z_axis:Axis,
}

impl System {

    pub fn new(command:&[Command<i32>]) -> Self {
        let x_axis = Axis::new(command,|c| &c.x_range);
        let y_axis = Axis::new(command,|c| &c.y_range);
        let z_axis = Axis::new(command,|c| &c.z_range);

        System{x_axis,y_axis,z_axis}
    }


    fn transform_cube(&self, cube:&Cube<i32>) -> AOCResult<Cube<usize>> {
        let x_range = self.x_axis.transform_range(&cube.x_range)?;
        let y_range = self.y_axis.transform_range(&cube.y_range)?;
        let z_range = self.z_axis.transform_range(&cube.z_range)?;

        Ok(Cube { x_range, y_range, z_range })
    }

    pub fn transform_command(&self, command:&Command<i32>) -> AOCResult<Command<usize>> {
        self.transform_cube(&command.cube).map(|cube| Command{light_state:command.light_state,cube })
    }

    pub fn volume_at(&self, coordinate:&(usize,usize,usize)) -> usize {
        let x_length = self.x_axis.length(coordinate.0);
        let y_length = self.y_axis.length(coordinate.1);
        let z_length = self.z_axis.length(coordinate.2);


        x_length * y_length * z_length
    }
}