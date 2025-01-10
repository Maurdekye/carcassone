use ggez::glam::vec2;

use crate::tile::{
    tile_definitions::{L_CURVE_ROAD, STRAIGHT_ROAD},
    MountingPair, Orientation, Tile,
};

#[test]
fn straight_road_mount_north_south() {
    let result = STRAIGHT_ROAD.validate_mounting(&STRAIGHT_ROAD, Orientation::North);
    assert_eq!(
        result,
        Some(vec![MountingPair {
            from_segment: 0,
            to_segment: 2,
        }])
    )
}

#[test]
fn straight_road_mount_east_west() {
    let result = STRAIGHT_ROAD.validate_mounting(&STRAIGHT_ROAD, Orientation::East);
    assert_eq!(
        result,
        Some(vec![
            MountingPair {
                from_segment: 0,
                to_segment: 0,
            },
            MountingPair {
                from_segment: 1,
                to_segment: 1,
            },
            MountingPair {
                from_segment: 2,
                to_segment: 2,
            },
        ],)
    )
}

#[test]
fn straight_road_curve_road_mount_west() {
    let result = STRAIGHT_ROAD.validate_mounting(&L_CURVE_ROAD, Orientation::West);
    assert_eq!(result, None);
}

#[test]
fn straight_road_curve_road_rotate_mount_west() {
    let mut curve_road = L_CURVE_ROAD.clone();
    curve_road.rotate();
    let result = STRAIGHT_ROAD.validate_mounting(&curve_road, Orientation::West);
    assert_eq!(
        result,
        Some(vec![
            MountingPair {
                from_segment: 2,
                to_segment: 2
            },
            MountingPair {
                from_segment: 1,
                to_segment: 1
            },
            MountingPair {
                from_segment: 0,
                to_segment: 0
            }
        ])
    );
}

#[test]
fn test_tile_definition() {
    use crate::tile::{
        Orientation, SegmentBorderPiece, SegmentDefinition, SegmentEdgePortion, SegmentType, Tile,
    };

    use Orientation::*;
    use SegmentBorderPiece::*;
    use SegmentDefinition::*;
    use SegmentEdgePortion::*;
    use SegmentType::*;
    
    let tile = Tile::new(
        vec![vec2(0.45, 0.45), vec2(0.55, 0.55)],
        vec![
            Segment {
                stype: Field,
                edges: vec![Edge(End, West), Edge(Beginning, North), Vert(0)],
            },
            Segment {
                stype: Road,
                edges: vec![Edge(Middle, West), Vert(0), Edge(Middle, North), Vert(1)],
            },
            Segment {
                stype: Field,
                edges: vec![
                    Edge(Beginning, West),
                    Vert(1),
                    Edge(End, North),
                    Edge(Full, East),
                    Edge(Full, South),
                ],
            },
        ],
    );
    dbg!(tile);
}
