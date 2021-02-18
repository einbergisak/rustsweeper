


#[cfg(test)]
mod tests{
    use crate::tile::Coordinate;
    use crate::game_container::GameContainer;


    #[test]
    fn array_coordinates_are_properly_set(){
        // Arrange
        let g = GameContainer::new("asd");
        let t = g.tile_array;
        assert_eq!(t[0][0].coordinates, Coordinate::new(0, 0));
        assert_eq!(t[5][2].coordinates, Coordinate::new(5, 2));
        assert_eq!(t[15][15].coordinates, Coordinate::new(15, 15));
    }

}