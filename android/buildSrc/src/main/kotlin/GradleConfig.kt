object GradleConfigs {
    const val compileSdk = 34
    const val minSdk = 26
    const val ndkVersion = "27.3.13750724"
    const val baseNamespace = "org.ibadalrahman"
    const val packageVersion = "0.6.0"

    fun subNamespace(sub: String) = "$baseNamespace.$sub"
}
