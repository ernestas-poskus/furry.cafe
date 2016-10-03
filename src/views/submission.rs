use iron::Request;

use std::borrow::Cow;
use maud::{Markup, PreEscaped};

use views;
use error;
use helper::*;
use views::layout::LayoutData;
use views::components::user::UserLink;
use views::components::form::*;
use views::components::button::*;
use models::submission::{Submission, SubmissionError, NewSubmission};
use models::user::User;
use middleware::authorization::{self, UserAuthorization};

pub fn index(subs: &[Submission], data: &LayoutData, req: &mut Request, user: Option<User>) -> Result<Markup, error::FurratoriaError> {
    let body = html! {
        h1 {
            { (user.as_ref().map(|x| format!("{} ", x.name.possessive())).unwrap_or(String::new())) "Gallery" }

            @if req.current_user_can(authorization::LoggedIn) && user.is_none() {
                a.btn.btn-primary.pull-xs-right href=(url!("/submissions/new")) "New Submission"
            }
        }

        div.submissions @for sub in subs {
            div a href=(url!(format!("/submissions/{}", sub.id))) {
                div.card {
                    img.card-img-top src=(match try!(sub.get_image()) {
                        Some(i) => try!(i.get_with_size(500, 500)).map(|x| x.get_path()).unwrap_or_else(|| String::from("/todo")),
                        None => String::from("/todo")
                    }) /
                    div.card-block {
                        h4.card-title (sub.title)
                        div.card-subtitle.text-muted {
                            "by "
                            ({PreEscaped(UserLink(&try!(sub.get_submitter())))})
                        }
                    }
                }
            }
        }
    };

    Ok(views::layout::application(Cow::Borrowed("Submissions"), body, data))
}

pub fn new(errors: Option<SubmissionError>, data: &LayoutData, sub: Option<&NewSubmission>) -> Result<Markup, error::FurratoriaError> {
    let body = html! {
        div.row div class="col-sm-6 offset-sm-3" {
            h1 { "Upload new Submission" }

            (PreEscaped(Form::new(FormMethod::Post, "/submissions/")
              .with_encoding("multipart/form-data")
              .with_fields(&[
                   &Input::new("Image", "sub_image")
                        .with_type("file")
                        .with_errors(errors.as_ref().map(|x| &x.image)),
                   &Input::new("Title", "sub_name")
                        .with_value(sub.as_ref().map(|x| &x.title).unwrap_or(&""))
                        .with_errors(errors.as_ref().map(|x| &x.title)),
                   &Textarea::new("Description", "sub_desc")
                        .with_value(sub.as_ref().map(|x| &x.description).unwrap_or(&""))
                        .with_errors(None),
                   &Select::new("Visibility", "sub_visibility")
                        .add_option("Public","0")
                        .add_option("Private", "2")
                        .with_selected(sub.as_ref().map(|x| x.get_visibility().as_str()).unwrap_or(&"")),
                   &Input::new("", "")
                        .with_value("Upload")
                        .with_type("submit")
                        .with_class("btn btn-primary")
              ])))
        }
    };

    Ok(views::layout::application(Cow::Borrowed("Register"), body, data))
}

pub fn show(sub: &Submission, data: &LayoutData, req: &mut Request) -> Result<Markup, error::FurratoriaError> {
    let image = match try!(sub.get_image()) {
        Some(i) => i,
        None => {
            return Err(error::FurratoriaError::Template(Box::new(error::FurratoriaError::NotFound)))
        }
    };

    let user = try!(sub.get_submitter());

    let body = html! {
        div.submission {
            div.row div class="col-md-10 offset-md-1" {
                div.submission.clearfix {
                    img src=(image.get_path()) alt=(format!("{}'s Submission", user.name)) /
                }

                div {
                    h1.title { (sub.title) }
                    span.uploader {
                        "Uploaded by "
                        (PreEscaped(UserLink(&user)))
                    }
                }
            }

            @if req.current_user_can(authorization::LoggedIn) {
                div.row div class="col-md-10 offset-md-1" {
                    div.sub_actions {
                        a.btn.btn-primary href=(url!(format!("/users/{}/edit", user.id))) "Favorit"
                        a.btn.btn-secondary href=(image.get_path()) "Full Size"
                        @if req.current_user_can(authorization::SameUserAuthAs(&user)) {
                            a.btn.btn-info href=(url!(format!("/submissions/{}/edit", sub.id))) "Edit"
                        }
                        a.btn.btn-danger href=(url!(format!("/users/{}/profile/edit", user.id))) "Signal"
                    }
                }
            }

            div.row div class="col-md-10 offset-md-1" {
                div.submission_description {
                    (views::markdown::parse(&sub.description))
                }
            }


        }
    };

    Ok(views::layout::application(Cow::Owned(format!("{} by {}", sub.title, user.name)), body, data))
}

pub fn edit(sub: &Submission, errors: Option<SubmissionError>, data: &LayoutData) -> Result<Markup, error::FurratoriaError> {
    let body = html! {
        div.row div class="col-sm-6 offset-sm-3" {
            h1 { "Update your Submission" }

            (PreEscaped(Form::new(FormMethod::Post, &format!("/submissions/{}", sub.id))
              .with_encoding("multipart/form-data")
              .with_fields(&[
                   &Input::new("Image", "sub_image")
                        .with_type("file")
                        .with_errors(errors.as_ref().map(|x| &x.image)),
                   &Input::new("Title", "sub_name")
                        .with_value(&sub.title[..])
                        .with_errors(errors.as_ref().map(|x| &x.title)),
                   &Textarea::new("Description", "sub_desc")
                        .with_value(&sub.description)
                        .with_errors(None),
                   &Select::new("Visibility", "sub_visibility")
                        .add_option("Public","0")
                        .add_option("Private", "2")
                        .with_selected(sub.get_visibility().as_str()),
                   &Input::new("", "")
                        .with_value("Update")
                        .with_type("submit")
                        .with_class("btn btn-primary")
              ])))
        }

        div.row div class="col-sm-6 offset-sm-3" {
            (PreEscaped(Button::new("Delete", &format!("/submissions/{}/delete", sub.id)).with_method(RequestMethod::Post)))
        }
    };

    Ok(views::layout::application(Cow::Borrowed("Register"), body, data))
}
