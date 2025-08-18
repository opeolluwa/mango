package com.eckko.app.mobile

interface Platform {
    val name: String
}

expect fun getPlatform(): Platform