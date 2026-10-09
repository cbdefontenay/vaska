# Application :

Cette application est une application **Android** qui permet de suivre les liens de partage sur les réseaux sociaux et d'enlever les trackers qui se trouvent dans les liens.
Sur le principe, c'est très simple. Cette application est crée avec les technologies suivantes :

- Dioxus et Rust.
- Tailwindcss.

# Ce que tu dois savoir et prendre en compte avant une mission :

1. Utilise toujours _Tailwindcss_, jamais de **CSS** pur, à moins que je ne te le demande moi-même dans une mission.
2. Ne jamais utiliser de **JS** dans le code, sauf si je te le demande moi-même dans une mission. Utilise toujours **Rust**.
3. Je n'aime pas les fichiers très longs en code, donc j'aime créer plusieurs components réutilisables où qui peuvent permettre de réduire la taille du code.
4. Ne crée **JAMAIS** du code dans un fichier `mod.rs`. Ces fichiers permettent simplement l'export des fonctions.
5. Reste **TOUJOURS** dans le `path` du projet `Vaska`, ne va jamais voir ailleur sur mon odinateur.

# Ta mission :

Dans le fichier `db_connect.rs` j'ai ajouté une connection SQLite à sqlx. Mais je pense qu'utiliser `fullstack` et desfonctions server, ce n'est pas nécessaire. Si tu penses que c'est nécessaire, alors change, sinon, enlève le fullstack. Mais je souhaite pouvoir ajouter un bouton `Enregistrer URL` à côté de celui qui permet de copier l'URL nettoyé. Les URLs enregistrés seront ensuite affichés dans la page `url_saver.rs`.
