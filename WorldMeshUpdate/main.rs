#[allow(dead_code)]

pub enum WorldMeshUpdate{
    PlaceBlock(&Block),
    BlockToEntity(&Block, &EntityBlock),
    EntityToBlock(&EntityBlock, &Block),
}
