package com.eckko.app.eckko

interface Platform {
    val name: String
}

expect fun getPlatform(): Platform