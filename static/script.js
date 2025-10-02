let currentQuestion = null;
let selectedOption = null;

// 标签页切换
function switchTab(tabName) {
    // 移除所有active类
    document.querySelectorAll('.tab-btn').forEach(btn => btn.classList.remove('active'));
    document.querySelectorAll('.menu-item').forEach(btn => btn.classList.remove('active'));
    document.querySelectorAll('.tab-content').forEach(content => content.classList.remove('active'));
    
    // 激活当前标签页
    event.target.classList.add('active');
    document.getElementById(tabName + '-tab').classList.add('active');
}

// 原有的随机装备功能
async function generateFullLoadout() {
    console.log('🎲 开始生成完整配置...');
    const fullLoadoutEl = document.getElementById('fullLoadout');
    const loadingIndicator = document.getElementById('loadingIndicator');
    
    // 先隐藏装备显示区域，显示加载指示器
    fullLoadoutEl.style.display = 'none';
    fullLoadoutEl.style.opacity = '0';
    loadingIndicator.style.display = 'block';
    
    // 显示加载状态
    const elements = ['fullMap', 'fullOperator', 'fullWeapon', 'fullHelmet', 'fullArmor'];
    const imageElements = ['fullMapImg', 'fullOperatorImg', 'fullWeaponImg', 'fullHelmetImg', 'fullArmorImg'];
    
    // 先隐藏所有图片
    imageElements.forEach(id => {
        const imgEl = document.getElementById(id);
        if (imgEl) {
            imgEl.style.display = 'none';
        }
    });
    
    try {
        // 读取过滤条件
        const classifiedOnly = document.getElementById('classifiedOnly')?.checked || false;
        const excludePistols = document.getElementById('excludePistols')?.checked || false;
        
        // 构建查询参数
        const params = new URLSearchParams();
        if (classifiedOnly) params.append('classified_only', 'true');
        if (excludePistols) params.append('exclude_pistols', 'true');
        
        const url = `/api/generate/loadout${params.toString() ? '?' + params.toString() : ''}`;
        console.log('📡 发送请求到:', url);
        
        const response = await fetch(url, {
            method: 'GET',
            headers: {
                'Accept': 'application/json',
                'Cache-Control': 'no-cache'
            }
        });
        
        console.log('📡 响应状态:', response.status, response.statusText);
        
        if (!response.ok) {
            throw new Error(`HTTP Error: ${response.status} ${response.statusText}`);
        }
        
        const data = await response.json();
        console.log('✅ 收到数据:', data);
        
        // 收集所有需要加载的图片Promise
        const imageLoadPromises = [];
        
        // 更新元素并收集图片加载Promise
        const updateLoadoutItem = (textId, imageId, placeholderId, item) => {
            const textEl = document.getElementById(textId);
            const imgEl = document.getElementById(imageId);
            const placeholderEl = document.getElementById(placeholderId);
            
            if (textEl && item) {
                const name = item.name || item;
                textEl.textContent = name;
                console.log(`✓ 更新 ${textId}: ${name}`);
            }
            
            if (imgEl && item?.image) {
                const imageUrl = '/' + item.image.replace(/^\/+/, '');
                
                // 显示占位符
                if (placeholderEl) {
                    placeholderEl.style.display = 'block';
                }
                imgEl.classList.add('loading');
                
                // 创建图片加载Promise
                const imageLoadPromise = new Promise((resolve) => {
                    imgEl.onload = () => {
                        console.log(`✓ 图片加载成功: ${item.image}`);
                        // 隐藏占位符，显示图片
                        if (placeholderEl) {
                            placeholderEl.style.display = 'none';
                        }
                        imgEl.classList.remove('loading');
                        resolve();
                    };
                    imgEl.onerror = () => {
                        console.warn(`⚠️ 图片加载失败: ${item.image}`);
                        if (placeholderEl) {
                            placeholderEl.style.display = 'none';
                        }
                        imgEl.style.display = 'none';
                        resolve(); // 即使失败也resolve，不阻止显示
                    };
                    imgEl.src = imageUrl;
                });
                
                imageLoadPromises.push(imageLoadPromise);
                console.log(`🔄 开始加载图片 ${imageId}: ${item.image}`);
            }
        };
        
        updateLoadoutItem('fullMap', 'fullMapImg', 'fullMapPlaceholder', data.map);
        updateLoadoutItem('fullOperator', 'fullOperatorImg', 'fullOperatorPlaceholder', data.operator);
        updateLoadoutItem('fullWeapon', 'fullWeaponImg', 'fullWeaponPlaceholder', data.primary_weapon);
        updateLoadoutItem('fullHelmet', 'fullHelmetImg', 'fullHelmetPlaceholder', data.helmet);
        updateLoadoutItem('fullArmor', 'fullArmorImg', 'fullArmorPlaceholder', data.armor);
        
        // 等待所有图片加载完成
        console.log('⏳ 等待所有图片加载完成...');
        await Promise.all(imageLoadPromises);
        
        // 显示所有成功加载的图片
        imageElements.forEach(id => {
            const imgEl = document.getElementById(id);
            if (imgEl && imgEl.complete && imgEl.naturalWidth > 0) {
                imgEl.style.display = 'block';
            }
        });
        
        // 隐藏加载指示器
        loadingIndicator.style.display = 'none';
        
        // 图片加载完成后，显示装备区域并添加淡入动画
        fullLoadoutEl.style.display = 'block';
        setTimeout(() => {
            fullLoadoutEl.style.transition = 'opacity 0.5s ease-in';
            fullLoadoutEl.style.opacity = '1';
        }, 50);
        
        console.log('🎉 完整配置生成成功，所有图片已加载');
        Toast.success('装备配置生成成功！', 2000);
        
    } catch (error) {
        console.error('❌ 生成失败:', error);
        console.error('❌ 错误详情:', {
            message: error.message,
            stack: error.stack,
            name: error.name
        });
        
        // 隐藏加载指示器
        loadingIndicator.style.display = 'none';
        
        // 如果出错，也要显示区域（显示错误信息）
        fullLoadoutEl.style.display = 'block';
        fullLoadoutEl.style.opacity = '1';
        
        elements.forEach(id => {
            const el = document.getElementById(id);
            if (el) {
                el.textContent = '生成失败';
                el.classList.remove('loading');
            }
        });
        
        // 使用Toast显示用户友好的错误信息
        const errorMsg = error.message.includes('fetch') ? 
            '网络连接失败，请检查网络或稍后重试' : 
            `生成失败: ${error.message}`;
            
        Toast.error(errorMsg, 5000);
    }
}

// 新增的三角洲高考功能
async function getRandomQuestion() {
    showLoading();
    try {
        // 添加时间戳防止缓存
        const timestamp = new Date().getTime();
        const response = await fetch(`/api/exam/question?_t=${timestamp}`, {
            cache: 'no-store'
        });
        const data = await response.json();
        displayQuestion(data);
    } catch (error) {
        console.error('获取题目失败:', error);
        alert('获取题目失败，请重试');
    }
}

function showLoading() {
    const examContainer = document.getElementById('examContainer');
    examContainer.style.display = 'block';
    
    document.getElementById('questionText').textContent = '正在加载题目...';
    document.getElementById('optionsContainer').innerHTML = '';
    document.getElementById('submitBtn').disabled = true;
    document.getElementById('resultPanel').classList.remove('show');
}

function displayQuestion(data) {
    currentQuestion = data.question;
    selectedOption = null;
    
    const examContainer = document.getElementById('examContainer');
    examContainer.style.display = 'block';
    
    document.getElementById('questionText').textContent = currentQuestion.question;
    
    // 生成选项
    const optionsContainer = document.getElementById('optionsContainer');
    optionsContainer.innerHTML = '';
    
    data.options_labeled.forEach((option, index) => {
        const optionEl = document.createElement('div');
        optionEl.className = 'option';
        optionEl.textContent = option;
        optionEl.onclick = () => selectOption(index, optionEl);
        optionsContainer.appendChild(optionEl);
    });
    
    document.getElementById('submitBtn').disabled = true;
    document.getElementById('resultPanel').classList.remove('show');
}

function selectOption(index, element) {
    // 移除之前的选择
    document.querySelectorAll('.option').forEach(opt => opt.classList.remove('selected'));
    
    // 标记当前选择
    element.classList.add('selected');
    selectedOption = index;
    
    // 启用提交按钮
    document.getElementById('submitBtn').disabled = false;
}

async function submitAnswer() {
    if (selectedOption === null || !currentQuestion) {
        alert('请先选择答案');
        return;
    }
    
    const submitBtn = document.getElementById('submitBtn');
    submitBtn.disabled = true;
    submitBtn.textContent = '提交中...';
    
    try {
        const response = await fetch('/api/exam/answer', {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json',
            },
            body: JSON.stringify({
                question_id: currentQuestion.id,
                selected_option: selectedOption
            })
        });
        
        const result = await response.json();
        if (result.error) {
            alert(result.error);
            return;
        }
        
        displayResult(result);
        
    } catch (error) {
        console.error('提交答案失败:', error);
        alert('提交答案失败，请重试');
    } finally {
        submitBtn.textContent = '提交答案';
    }
}

function displayResult(result) {
    const resultPanel = document.getElementById('resultPanel');
    const resultTitle = document.getElementById('resultTitle');
    const correctAnswer = document.getElementById('correctAnswer');
    
    resultPanel.classList.add('show');
    
    if (result.is_correct) {
        resultPanel.className = 'result-panel show correct';
        resultTitle.className = 'result-title correct';
        resultTitle.textContent = '🎉 回答正确！';
    } else {
        resultPanel.className = 'result-panel show wrong';
        resultTitle.className = 'result-title wrong';
        resultTitle.textContent = '❌ 回答错误';
        correctAnswer.textContent = `正确答案：${result.correct_option}`;
    }
    
    // 标记选项的正确性
    const options = document.querySelectorAll('.option');
    options.forEach((option, index) => {
        if (index === result.correct_answer) {
            option.classList.add('correct');
        } else if (index === selectedOption && !result.is_correct) {
            option.classList.add('wrong');
        }
        option.onclick = null; // 禁用点击
    });
}

// 音乐播放器功能
let currentSongIndex = 0;
let isPlaying = false;
let isPlayerVisible = false;
let playlists = [];
let currentPlaylist = [];
let currentPlaylistId = '';
let currentSong = null;

let audioPlayer;
let playBtn;
let musicPlayer;

// 加载播放列表数据
async function loadPlaylistData() {
    try {
        const response = await fetch('/api/music/playlist');
        if (!response.ok) {
            throw new Error(`HTTP ${response.status}: ${response.statusText}`);
        }
        const data = await response.json();
        if (data.error) {
            throw new Error(data.error);
        }
        playlists = data.playlists || [];
        initializePlaylistSelector();
        if (playlists.length > 0) {
            switchPlaylist(playlists[0].id);
        }
    } catch (error) {
        console.error('加载播放列表失败:', error);
        alert('无法加载播放列表，请检查服务器配置');
        playlists = [];
        initializePlaylistSelector();
    }
}



// 初始化播放列表选择器
function initializePlaylistSelector() {
    const select = document.getElementById('playlistSelect');
    select.innerHTML = '<option value="">选择播放列表...</option>';
    
    playlists.forEach(playlist => {
        const option = document.createElement('option');
        option.value = playlist.id;
        option.textContent = `${playlist.name} (${playlist.songs.length}首)`;
        select.appendChild(option);
    });
}

// 切换播放列表
function switchPlaylist(playlistId) {
    if (!playlistId) return;
    
    const playlist = playlists.find(p => p.id === playlistId);
    if (!playlist) return;
    
    currentPlaylistId = playlistId;
    currentPlaylist = playlist.songs;
    currentSongIndex = 0;
    
    document.getElementById('playlistSelect').value = playlistId;
    renderPlaylist();
    
    if (currentPlaylist.length > 0) {
        loadSong(0);
    }
}

// 切换播放器显示/隐藏
function toggleMusicPlayer() {
    if (!musicPlayer) {
        musicPlayer = document.getElementById('musicPlayer');
        if (!musicPlayer) return;
    }
    
    if (!isPlayerVisible) {
        // 显示播放器
        musicPlayer.classList.add('show');
        isPlayerVisible = true;
        
        // 更新按钮提示文字
        const trigger = document.querySelector('.music-trigger');
        if (trigger) trigger.title = '隐藏音乐播放器';
    } else {
        // 隐藏播放器
        musicPlayer.classList.remove('show');
        isPlayerVisible = false;
        
        // 更新按钮提示文字
        const trigger = document.querySelector('.music-trigger');
        if (trigger) trigger.title = '打开音乐播放器';
    }
}

// 显示播放器（保留兼容性）
function showPlayer() {
    toggleMusicPlayer();
}

// 加载歌曲
function loadSong(index) {
    if (index >= 0 && index < currentPlaylist.length) {
        currentSongIndex = index;
        currentSong = currentPlaylist[index];
        
        const songTitle = document.getElementById('songTitle');
        const songArtist = document.getElementById('songArtist');
        
        if (songTitle) songTitle.textContent = currentSong.title;
        if (songArtist) songArtist.textContent = currentSong.artist;
        
        if (!audioPlayer) {
            audioPlayer = document.getElementById('audioPlayer');
        }
        if (audioPlayer) {
            audioPlayer.src = currentSong.file;
        }
        
        // 更新歌词
        updateLyrics();
        
        // 更新播放列表活动状态
        updatePlaylistActive();
        
        // 更新按钮状态
        updateActionButtons();
    }
}

// 更新歌词显示
function updateLyrics() {
    const lyricsContent = document.getElementById('lyricsContent');
    if (currentSong && currentSong.lyrics && currentSong.lyrics.length > 0) {
        lyricsContent.innerHTML = currentSong.lyrics
            .map((line, index) => `<p data-line="${index}">${line}</p>`)
            .join('');
    } else {
        lyricsContent.innerHTML = '<p>暂无歌词</p>';
    }
}

// 更新操作按钮状态
function updateActionButtons() {
    const bilibiliBtn = document.getElementById('bilibiliBtn');
    const downloadBtn = document.getElementById('downloadBtn');
    
    if (currentSong) {
        bilibiliBtn.style.display = currentSong.bilibili_url ? 'flex' : 'none';
        downloadBtn.style.display = 'flex';
    } else {
        bilibiliBtn.style.display = 'none';
        downloadBtn.style.display = 'none';
    }
}

// 打开B站链接
function openBilibiliLink() {
    if (currentSong && currentSong.bilibili_url) {
        window.open(currentSong.bilibili_url, '_blank');
    }
}

// 下载当前歌曲
function downloadCurrentSong() {
    if (currentSong) {
        const link = document.createElement('a');
        link.href = currentSong.file;
        link.download = `${currentSong.title} - ${currentSong.artist}.mp3`;
        link.style.display = 'none';
        document.body.appendChild(link);
        link.click();
        document.body.removeChild(link);
    }
}

// 渲染播放列表
function renderPlaylist() {
    const playlistContainer = document.getElementById('playlist');
    playlistContainer.innerHTML = '';
    
    currentPlaylist.forEach((song, index) => {
        const item = document.createElement('div');
        item.className = 'playlist-item';
        item.onclick = () => {
            loadSong(index);
            if (isPlaying) {
                audioPlayer.play();
            }
        };
        
        item.innerHTML = `
            <div class="song-number">${index + 1}</div>
            <div class="song-details">
                <div class="song-name">${song.title}</div>
                <div class="song-duration">${song.duration}</div>
            </div>
        `;
        
        playlistContainer.appendChild(item);
    });
}

// 更新播放列表活动状态
function updatePlaylistActive() {
    const items = document.querySelectorAll('.playlist-item');
    items.forEach((item, index) => {
        if (index === currentSongIndex) {
            item.classList.add('active');
        } else {
            item.classList.remove('active');
        }
    });
}

// 播放/暂停切换
function togglePlay() {
    if (!audioPlayer) {
        audioPlayer = document.getElementById('audioPlayer');
        if (!audioPlayer) return;
    }
    
    if (!playBtn) {
        playBtn = document.getElementById('playBtn');
        if (!playBtn) return;
    }
    
    if (isPlaying) {
        audioPlayer.pause();
        playBtn.textContent = '▶';
        isPlaying = false;
    } else {
        audioPlayer.play().then(() => {
            playBtn.textContent = '⏸';
            isPlaying = true;
        }).catch((error) => {
            console.error('播放失败:', error);
            alert('音频文件加载失败，请检查文件路径');
        });
    }
}

// 上一首
function previousSong() {
    const prevIndex = currentSongIndex - 1;
    const newIndex = prevIndex < 0 ? currentPlaylist.length - 1 : prevIndex;
    loadSong(newIndex);
    if (isPlaying && audioPlayer) {
        audioPlayer.play();
    }
}

// 下一首
function nextSong() {
    const nextIndex = (currentSongIndex + 1) % currentPlaylist.length;
    loadSong(nextIndex);
    if (isPlaying && audioPlayer) {
        audioPlayer.play();
    }
}

// 设置音量
function setVolume(volume) {
    if (!audioPlayer) {
        audioPlayer = document.getElementById('audioPlayer');
    }
    if (audioPlayer) {
        audioPlayer.volume = volume / 100;
    }
}

// 设置播放进度
function setProgress(event) {
    if (!audioPlayer) {
        audioPlayer = document.getElementById('audioPlayer');
    }
    if (audioPlayer && audioPlayer.duration) {
        const progressBar = event.currentTarget;
        const rect = progressBar.getBoundingClientRect();
        const pos = (event.clientX - rect.left) / rect.width;
        audioPlayer.currentTime = pos * audioPlayer.duration;
    }
}

// 更新播放进度
function updateProgress() {
    if (!audioPlayer) {
        audioPlayer = document.getElementById('audioPlayer');
    }
    if (audioPlayer && audioPlayer.duration) {
        const progress = (audioPlayer.currentTime / audioPlayer.duration) * 100;
        const progressFill = document.getElementById('progressFill');
        const progressHandle = document.getElementById('progressHandle');
        const currentTime = document.getElementById('currentTime');
        
        if (progressFill) progressFill.style.width = progress + '%';
        if (progressHandle) progressHandle.style.left = progress + '%';
        
        if (currentTime) {
            currentTime.textContent = formatTime(audioPlayer.currentTime);
        }
    }
}

// 更新总时长
function updateDuration() {
    if (!audioPlayer) {
        audioPlayer = document.getElementById('audioPlayer');
    }
    if (audioPlayer && audioPlayer.duration) {
        const totalTime = document.getElementById('totalTime');
        if (totalTime) {
            totalTime.textContent = formatTime(audioPlayer.duration);
        }
    }
}

// 格式化时间
function formatTime(seconds) {
    const mins = Math.floor(seconds / 60);
    const secs = Math.floor(seconds % 60);
    return `${mins}:${secs.toString().padStart(2, '0')}`;
}

// Cloudflare 兼容性和初始化
function initializeApp() {
    console.log('🚀 应用初始化开始...');
    
    // 检查关键元素是否存在
    const criticalElements = [
        'fullLoadout', 'fullMap', 'fullOperator', 'fullWeapon', 'fullHelmet', 'fullArmor'
    ];
    
    let missingElements = [];
    criticalElements.forEach(id => {
        if (!document.getElementById(id)) {
            missingElements.push(id);
        }
    });
    
    if (missingElements.length > 0) {
        console.warn('⚠️ 缺少关键元素:', missingElements);
    } else {
        console.log('✅ 所有关键元素已就绪');
    }
    
    // 加载音乐播放列表
    loadPlaylistData();
    
    // 延迟执行初始装备生成，确保 Cloudflare 脚本不干扰
    setTimeout(() => {
        console.log('🎲 执行初始装备生成...');
        generateFullLoadout();
    }, 1000); // 增加延迟到1秒，确保 CF 脚本加载完成
}

// 设置事件监听器
function setupEventListeners() {
    // 获取DOM元素
    audioPlayer = document.getElementById('audioPlayer');
    playBtn = document.getElementById('playBtn');
    musicPlayer = document.getElementById('musicPlayer');
    
    // 确保元素存在后再添加事件监听器
    if (audioPlayer) {
        // 监听音频事件
        audioPlayer.addEventListener('ended', () => {
            nextSong();
        });
        
        audioPlayer.addEventListener('pause', () => {
            if (playBtn) playBtn.textContent = '▶';
            isPlaying = false;
        });
        
        audioPlayer.addEventListener('play', () => {
            if (playBtn) playBtn.textContent = '⏸';
            isPlaying = true;
        });
    }
    
    // 点击页面其他地方隐藏播放器（可选）
    document.addEventListener('click', (event) => {
        if (isPlayerVisible && 
            musicPlayer && !musicPlayer.contains(event.target) && 
            !document.querySelector('.music-trigger').contains(event.target)) {
            // 可以选择是否点击外部隐藏播放器
            musicPlayer.classList.remove('show');
            isPlayerVisible = false;
        }
    });
}

// 确保在 DOM 完全加载后再执行
if (document.readyState === 'loading') {
    document.addEventListener('DOMContentLoaded', () => {
        initializeApp();
        setupEventListeners();
    });
} else {
    // DOM 已经加载完成
    initializeApp();
    setupEventListeners();
}

// 额外的保险措施：在 window.onload 后再次尝试
window.addEventListener('load', () => {
    setTimeout(() => {
        console.log('🔄 Window load 完成，确保功能正常...');
        // 如果元素存在但内容为空，重新生成
        const fullMapEl = document.getElementById('fullMap');
        if (fullMapEl && (!fullMapEl.textContent || fullMapEl.textContent.includes('生成中'))) {
            console.log('🔄 检测到内容异常，重新生成...');
            generateFullLoadout();
        }
    }, 500);
});