#import "template.typ": *

// Initialize the project
#show: project.with(
  title: "Conclusion",
  authors: (
    "Max Fehlinger",
    "Foxx Pinkerton",
  ),
  date: datetime.today().display("[month repr:long] [day], [year]"),
)

= Evaluation of the Product <eval>
Considering the amount of time to develop this product, nearly all expectations were fulfilled. The
application provides a trip planner which can both get a travel plan from the official API, as well
as Google Maps, so the user can decide which way to go. However, a few features are still missing:

- Saving a planned trip and notifying the user of transfers
- Listing stops nearby
- Showing trips/stops on a map
- Make the authentication process easier:
  - Getting a Google Maps API Token (e.g. through OAuth)
  - Getting a Winnipeg Transit API Token (e.g. integrated interface)

== Feedback of the client
As this project was developed to fill personal needs, we -- as developers -- are our own clients are
satisfied with the solution we produced. It was a very ambitious project with many features and we
managed to implement enough, to deliver a Minimum Viable Product.

== Other feedback
The User interface could be improved, to make the application more accessible. This means, changing
up the colours to make it easier to read the text, or to get help on specific options, when hovering
over parts of the application.

= Future Development
// Also include explanations (why?)
In the future there are a lot of features that could be implemented. The main one would be to create
a mobile application, as soon as tauri version 2.0 is released. This would mean, that the
application could actually be used in an efficient manner in the real world. After that, the
currently not implemented features listed in @eval should also be implemented to make this
application feature-complete and useable.

