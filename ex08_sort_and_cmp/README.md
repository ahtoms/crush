
# Sorting and comparing

We have the luxury of being able to use the defined equality and partial equality trait
associated with inbuilt types in rust. But what about our defined structs in rust?

We have the following struct

```
struct Album {
    id: usize,
    name: String,
    artist: String,
    year: u32,
    no_tracks: u32,
}
```

We are able to use `#[derive(eq)]` if we want to implicit lexigraphical ordering derived from the properties of the struct. 
However, this may not make any sense for the problem at hand. With this case, we have a list of 
`Album`s which we will order by year, artist name and the number of tracks.

Your program must be able to query an existing list of album as well as check if an album entered by the user already exists
within the list of albums.

The application will contain the following commands:

FIND, example: `find (albums | artists) by (id | album | artist | year | no_tracks)`
INSERT, example `insert 'album name' 'artist name' 'year' 'no_tracks'`


Example usage (assume an in built :

```
./album_query


```
