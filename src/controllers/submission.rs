use iron::prelude::*;
use iron::status;
use iron::headers::ContentType;
use iron::modifiers::Redirect;
use iron_login::User as UserTrait;

use error::{self};
use views;
use models;
use views::layout::LayoutData;
use models::user::User;
use models::submission;

pub fn index(req: &mut Request) -> IronResult<Response> {
    let sub_list = try!(models::submission::last(20));

    let data = LayoutData::from_request(req);
    let mut resp = Response::with((status::Ok, template!(views::submission::index(&sub_list, &data))));
    resp.headers.set(ContentType::html());
    Ok(resp)
}

pub fn new(req: &mut Request) -> IronResult<Response> {
    let data = LayoutData::from_request(req);
    let mut resp = Response::with((status::Ok, template!(views::submission::new(None, &data, None))));
    resp.headers.set(ContentType::html());
    Ok(resp)
}

pub fn create(req: &mut Request) -> IronResult<Response> {
    use params::{Params, Value};
    use models::submission::Submission;

    let user = User::get_login(req).get_user().unwrap();
    let data = LayoutData::from_request(req);

    let map = req.get_ref::<Params>().unwrap();

    let sub_name = match map.get("sub_name") {
        Some(&Value::String(ref name)) => Some(&name[..]),
        _ => None
    };

    let sub_desc = match map.get("sub_desc") {
        Some(&Value::String(ref name)) => Some(&name[..]),
        _ => None
    };

    let image = match map.get("sub_image") {
        Some(&Value::File(ref file)) => Some(file),
        _ => None
    };

    let new_submission = match models::submission::NewSubmission::new(&user, image, sub_name, sub_desc) {
        Ok(new_submission) => new_submission,
        Err((err, new_submission)) => {
            let mut resp = Response::with((status::Ok, template!(views::submission::new(Some(err), &data, Some(&new_submission)))));
            resp.headers.set(ContentType::html());
            return Ok(resp);
        }
    };

    let id = try!(Submission::create_from(new_submission));

    // TODO: Add config for url?
    return Ok(Response::with(temp_redirect!(format!("/submissions/{}", id))));
}

pub fn show(req: &mut Request) -> IronResult<Response> {
    let submission = try!(find_by_id!(req, "id", submission));

    let data = LayoutData::from_request(req);
    let mut resp = Response::with((status::Ok, template!(views::submission::show(&submission, &data))));
    resp.headers.set(ContentType::html());
    Ok(resp)
}

pub fn edit(req: &mut Request) -> IronResult<Response> {
    let data = LayoutData::from_request(req);

    let submission = try!(find_by_id!(req, "id", submission));

    let mut resp = Response::with((status::Ok, template!(views::submission::edit(&submission, None, &data))));
    resp.headers.set(ContentType::html());
    Ok(resp)
}

pub fn update(req: &mut Request) -> IronResult<Response> {
    use params::{Params, Value};

    let data = LayoutData::from_request(req);

    let submission = try!(find_by_id!(req, "id", submission));

    let map = req.get_ref::<Params>().unwrap();

    let sub_name = match map.get("sub_name") {
        Some(&Value::String(ref name)) => Some(&name[..]),
        _ => None
    };

    let sub_desc = match map.get("sub_desc") {
        Some(&Value::String(ref name)) => Some(&name[..]),
        _ => None
    };

    let image = match map.get("sub_image") {
        Some(&Value::File(ref file)) => Some(file),
        _ => None
    };

    let update_submission = match models::submission::UpdateSubmission::new(image ,sub_name, sub_desc) {
        Ok(update_submission) => update_submission,
        Err(err) => {
            let mut resp = Response::with((status::Ok, template!(views::submission::edit(&submission, Some(err), &data))));
            resp.headers.set(ContentType::html());
            return Ok(resp);
        }
    };

    try!(submission.update(&update_submission));

    return Ok(Response::with((status::SeeOther, Redirect(url!(format!("/submissions/{}", submission.id))))))
}

pub fn delete(req: &mut Request) -> IronResult<Response> {
    let submission = try!(find_by_id!(req, "id", submission));

    try!(submission.delete());

    return Ok(Response::with(temp_redirect!("/submissions/")));
}
