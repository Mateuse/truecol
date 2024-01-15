<template>
    <nav class="nav-links-nav">
        <div class="title">
            <span>True COL</span>
        </div>
        <button @click="toggleMenu" class="menu-toggle-btn" v-if="isMobile">
            <img v-if="!menuOpen" src="../assets/menu.svg">
        </button>
        <ul v-show="!menuOpen && !isMobile" class="nav-links-ul">
            <li>
                <router-link to="/" custom v-slot="{ navigate, isActive }">
                    <a :class="{ active: isActive }" @click="navigate">Home</a>
                </router-link>
            </li>
            <li>
                <router-link to="/about" custom v-slot="{ navigate, isActive }">
                    <a :class="{ active: isActive }" @click="navigate">About</a>
                </router-link>
            </li>
        </ul>
    </nav>
    <nav v-show="menuOpen || !isMobile" class="nav-links-mobile">
        <button @click="toggleMenu" class="close-menu-btn" v-if="isMobile">
            <img src="../assets/x.svg">
        </button>
        <ul>
            <li>
                <router-link to="/" custom v-slot="{ navigate, isActive }">
                    <a :class="{ active: isActive }" @click="navigate">Home</a>
                </router-link>
            </li>
            <li>
                <router-link to="/about" custom v-slot="{ navigate, isActive }">
                    <a :class="{ active: isActive }" @click="navigate">About</a>
                </router-link>
            </li>
        </ul>
    </nav>
</template>
  
<script>
import feather from 'feather-icons';

export default {
    name: 'NavBar',
    data() {
        return {
            menuOpen: false,
            isMobile: false
        };
    },
    methods: {
        toggleMenu() {
            this.menuOpen = !this.menuOpen;
        },
        checkMobile() {
            this.isMobile = window.innerWidth < 768;
        }
    },
    mounted() {
        feather.replace();
    },
    created() {
        window.addEventListener('resize', this.checkMobile);
        this.checkMobile();
    },
    beforeUnmount() {
        window.removeEventListener('resize', this.checkMobile);
    }
}
</script>
  
<style scoped lang="scss">
.nav-links-nav {
    display: flex;
    justify-content: space-between;
    align-items: center;
    box-shadow: 0 0 5px 0 rgba(0, 0, 0, 0.2);
    padding: 2rem 0 2rem 4rem;

    @media screen and (max-width: 768px) {
        padding: 1rem 2rem;
    }

    .title {
        display: flex;

        span {
            font-weight: 700;
            font-size: 2rem;

            @media screen and (max-width: 468px) {
                font-size: 1.5rem;
            }
        }
    }

    .menu-toggle-btn {
        display: none;
        background: none;
        border: none;
        font-size: 2rem;
        cursor: pointer;

        @media screen and (max-width: 768px) {
            display: block;
        }
    }

    .nav-links-ul {
        display: flex;
        list-style: none;
        text-decoration: none;
        margin-right: 4rem;

        li {
            font-weight: 500;
            font-size: 1rem;
            margin: 0 1rem;
            cursor: pointer;
            transition: transform 0.3s ease, color 0.3s ease;

            &:hover {
                transform: scale(1.2);
                color: $color-highlight-text;
            }
        }
    }

    .active {
        font-weight: 900;
        text-decoration: underline;
    }
}

.nav-links-mobile {
    display: none;

    @media screen and (max-width: 768px) {
        display: block;
        width: 60%;
        height: 100%;
        position: absolute;
        top: 0;
        right: 0;
        background-color: white;
        box-shadow: 2px 0 5px rgba(0, 0, 0, 0.3);

        button {
            margin: 1rem;
        }

        ul {
            list-style: none;
        }

        li {
            display: flex;
            align-items: center;
            padding: 1rem 2rem 1rem 2rem;
            cursor: pointer;
        }

        .active {
            font-weight: 900;
            text-decoration: underline;
        }
    }

    .close-menu-btn {
        background: none;
        border: none;
    }
}

.icon {
    width: 30px;
    height: 30px;
}
</style>