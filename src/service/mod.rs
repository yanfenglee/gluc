
use cgm_service::CgmService;

mod cgm_service;

lazy_static! {
   pub static ref CGM_SERVICE: CgmService = CgmService{};
}
