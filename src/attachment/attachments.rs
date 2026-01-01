use super::captured::*;

//
// Attachments
//

/// Attachments.
pub trait Attachments {
    /// All attachments.
    fn attachments(&self) -> impl Iterator<Item = &CapturedAttachment>;

    /// All attachments of a type.
    fn attachments_of_type<'own, AttachmentT>(&'own self) -> impl Iterator<Item = &'own AttachmentT>
    where
        AttachmentT: 'static,
    {
        self.attachments()
            .filter_map(|attachment| attachment.downcast_ref())
    }

    /// First attachment of a type.
    fn attachment_of_type<'own, AttachmentT>(&'own self) -> Option<&'own AttachmentT>
    where
        AttachmentT: 'static,
    {
        self.attachments_of_type().next()
    }
}
