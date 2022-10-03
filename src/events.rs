use super::*; 


#[repr(C,packed(1))]
#[derive(Copy,Clone,Default,Debug)]
pub struct EventHeaderTest{
    code:CARD8,
    detail:CARD8,
    sequence_number:CARD16,
    padding:[u8;28]
}



#[repr(C,packed(1))]
#[derive(Copy,Clone,Default,Debug)]
pub struct EventHeader{
    code:CARD8,
    detail:CARD8,
    sequence_number:CARD16,
}

#[repr(C,packed(1))]
#[derive(Copy,Clone,Default)]
struct GenericEvent{
    header:EventHeader,
    time:CARD32,
    root:Atom, 
    event:Atom,
    child:Atom, 
    root_x:CARD16,
    root_y:CARD16, 
    event_x:CARD16, 
    event_y:CARD16, 
    state:CARD16, 
    same_screen:BOOL,
    unuused:CARD8,
}


#[test]
pub fn event_size(){
    assert_eq!( 32, std::mem::size_of::<GenericEvent>() );
}