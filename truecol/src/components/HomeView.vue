<template>
    <div class="home">
        <h1>Home</h1>
        <div v-if="cities">
            <p>{{ cities }}</p>
        </div>
    </div>
</template>

<script>
export default {
    name: "HomeView", data() {
        return {
            cities: null, 
        };
    },
    created() {
        this.fetchData();
    },
    methods: {
        async fetchData() {
            try {
                console.log(process.env.VUE_APP_API_GET_CITIES)
                const response = await fetch(process.env.VUE_APP_API_GET_CITIES);
                if (!response.ok) {
                    throw new Error('Network response was not ok');
                }
                this.cities = await response.json();
            } catch (error) {
                console.error('Fetch error:', error);
            }
        }
    }
}
</script>

<style scoped lang="scss"></style>