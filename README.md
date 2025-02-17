The goal of `overseerr-sonarr` is to allow automatic setting of 1) the root folder 2) the quality profile, and 3) series type based on certain criteria.

Overseerr allows selecting one quality profile for standard and one for anime. Additionally all anime is added as series type "Anime / Absolute" without choice.

First you need to create the tag in sonarr in a way it will not remove it during housekeeping:

* Go to `Settings` -> `Tags`.
* Create an `Auto Tagging`.
* Enter name `overseerr-queue` and tags `overseerr-queue`.
* Add a condition with Tag with name and tag `overseerr-queue`.

Now to make use of it, add the following settings in Overseerr:

* Apply `tv` and `overseerr-queue` tags for standard shows.
* Apply `anime` and `overseerr-queue` tags for anime shows.
* Disable `Enable Automatic Search`.

Currently the behavior is:
* All anime (determined by `ANMIE_TAG_NAMES`) is set to series type "Standard".
* Shows in `ADDITIONAL_LANGUAGE` get a different root folder (`ADDITIONAL_LANGUAGE_ROOT_FOLDER`).
* Shows in `ADDITIONAL_LANGUAGE` get a different quality profile (`ADDITIONAL_LANGUAGE_QUALITY_PROFILE`).

# TODO
Currently this is not really customizable but if someone shows interest I'd be happy to improve it :)
