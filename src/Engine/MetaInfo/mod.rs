

/// #Description: 
///     This contains meta information of the application. 
///     This will always be loaded at the start of the application 
///     and is not associated with any specific game save.
///
///     Data included here can range from settings to total play time.
pub struct MetaInfo
{

}

impl MetaInfo
{
    pub fn Create() -> Self
    {
        Self{}
    }
}
