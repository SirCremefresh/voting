# Voting
A simple toll i created to learn some rust. It let's you create votings and add users where the admin is able to set the current active vote which the users will be able to vote to.

# Build
```
docker build . -t gcr.io/sircremefresh/voting:v0.0.1   
docker push gcr.io/sircremefresh/voting:v0.0.2   
```

# Api
// TODO: implement 418 i am a tea pot

type Poll = {
    name: string
    description: string
}

# Open
POST: /api/votings: {
    name: string
    polls: Poll[]
} -> {
    votingId: string
    adminKey: string
}

# Admin
HEADER: AUTHENTICATION: string
POST: /api/votings/{votingId}/user: {
    name: string
} -> {
    userKey: string
}

Schnittstelle um die aktive umfrage zu setzten
HEADER: AUTHENTICATION: string
PUT: /api/votings/{votingId}/polls/active: {
    pollIndex: number
} -> {
}

HEADER: AUTHENTICATION: string
GET: /api/votings/{votingId}/polls: {
    votingId: string
} -> {
    polls: Poll[]
}


# User

HEADER: AUTHENTICATION: string
GET: /api/votings/{votingId}/polls/active: {
} -> {
    pollIndex: number
    poll: Poll
    voted: boolean
}


HEADER: AUTHENTICATION: string
POST: /api/votings/{votingId}/polls/{pollIndex}/vote: {
    decision : 'YOP' | 'NOP' | 'NIL'
} -> {
} 
throws
    - 400 {
        reason: 'VOTED_ALREADY' | 'VOTE_NOT_ACTIVE'
    }

  
NOT_VOTED  
ACCEPTED  
DECLINED  
DRAW  