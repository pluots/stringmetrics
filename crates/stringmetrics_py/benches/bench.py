import inspect
import subprocess
import timeit

tests = (
    ("", "not empty", "left empty"),
    ("not empty", "", "right empty"),
    ("kitten", "sitting", "short"),
    ("an orange cat", "an auburn bat", "medium"),
    (
        "the quick brown fox jumped over the lazy dog",
        "a slow brown lox jumped under the heavy log",
        "long",
    ),
    (
        inspect.cleandoc(
            """
    Lorem ipsum dolor sit amet, consectetur adipiscing elit. Ut non dapibus
    felis, at molestie quam. Cras sed quam diam. Suspendisse tempor dolor sed
    dignissim luctus. Donec nec vestibulum sem. Ut ac orci ut augue lobortis
    condimentum. Aenean vestibulum maximus nunc, sit amet iaculis enim viverra
    et. Suspendisse imperdiet enim at luctus eleifend. Proin sagittis felis sit
    amet est egestas, eu maximus urna efficitur. Aliquam nec nunc et eros
    lacinia tincidunt. Nullam blandit laoreet eros, commodo vehicula augue
    eleifend at. Suspendisse augue lorem, pretium sit amet ullamcorper vel,
    rhoncus ut mi. Donec non velit hendrerit, faucibus urna ac, suscipit risus.
    Ut porta nulla at tincidunt lacinia. Sed neque nisi, tristique sed ornare
    id, accumsan at magna. Fusce non neque dapibus, elementum nisi interdum,
    convallis sem. Suspendisse potenti. Etiam lectus lacus, tempus at maximus
    eu, tempus eget dolor. Ut erat tellus, rhoncus in eleifend ut, rutrum ut
    odio. Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed
    consectetur ac mauris ut ultrices. Sed dapibus sapien risus, ac suscipit
    ligula molestie at. Duis sit amet laoreet magna.
    """
        ).replace("\n", " "),
        inspect.cleandoc(
            """
    Lorem ipsum dolor sit amet, consecatetur adipiscing elit. Ut non dapibus
    felis, at molestie qzam. Cras sed qusam diam. Suspendisse tempor dolor sed
    dignistim luctus. Donec nec vestibulum sem. Ut ac orci ut augue lobortis
    condimetum. Aenean vestibulum maximus nunc, sit amet iaculis enim viverra
    et. Suspendisse imperdiet enim at lucstus eleifend. Proin sagittis felis sit
    amet est egestas, eu maximus urna efficitur. Aliquam nec nunc et eros
    lacinia tincidunt. Nullam blandit laoreet eros, commodo vehdicula augue
    eleiftnd at. Suspendisse augue lorem, pretium sit amet ullamcorper vel,
    rhoincus ut mi. Donec
    non velit hendrerit, faucibus urna ac, suscipit risus. Ut porta nulla at
    tincidut lacinia. Sed neque nisi, tristique sed ornare id, accumsan at magna.
    Fusce non nezue daibus, elementum nisi interdum, convallis sem. Suspendisse
    potenti. Etiam lectus lacus, tempus at maximus eu, tempus eget dolor. Ut erat
    tellus, rhoncus in eleifend ut, rutrrum ut odio. Lorem ipsum dolor sit amet,
    conszctetur adipiscing elit. Sed consectetdur ac mauris ut ultrices. Szd dapibus
    sapien risus, ac ssscipit ligula molestie at. Duis sit amet laoreet magna.
    """
        ).replace("\n", " "),
        "extra long",
    ),
)

setups = (
    ("from jellyfish import levenshtein_distance as testfn", "jellyfish"),
    ("from stringmetrics import levenshtein as testfn", "stringmetrics"),
    ("from rapidfuzz.distance.Levenshtein import distance as testfn", "rapidfuzz"),
)


def si_format(num: float) -> str:
    shifts = 0
    while num < 1:
        num = num * 1000
        shifts -= 1

    pfx = {0: "", -1: "m", -2: "u", -3: "n", -4: "p"}[shifts]

    return f"{num:.2f} {pfx}s"


def main():
    for str0, str1, size in tests:
        for setup, name in setups:
            stmt = f"testfn('{str0}', '{str1}')"

            loops, time = timeit.Timer(stmt, setup).autorange()
            time_str = si_format(time / loops)
            name += ":"
            print(f"{size: <14} {name: <15} {time_str: >9} per loop ({loops} loops)")


if __name__ == "__main__":
    main()
