object GradleConfigs {
    const val compileSdk = 34
    const val minSdk = 26
    const val ndkVersion = "27.3.13750724"
    const val baseNamespace = "org.ibadalrahman"
    const val packageVersion = "0.6.1"

    // Maven coordinate for Central publishing. Intentionally differs from
    // baseNamespace (the Kotlin/Android package): io.github.<org> is the
    // free-to-verify Central namespace, while the code package stays put.
    const val mavenGroup = "io.github.ibad-al-rahman"
    const val projectUrl = "https://github.com/ibad-al-rahman/miqat"
    const val inceptionYear = "2024"

    fun subNamespace(sub: String) = "$baseNamespace.$sub"
}
