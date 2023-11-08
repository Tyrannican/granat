package stores

// TODO: Add Expiry at some point
type StringStore struct {
    store map[string]string
}

func (s *StringStore) Get(key string) string {
    if val, ok := s.store[key]; ok {
        return val
    }

    return ""
}

func (s *StringStore) Set(key, value string) {
    s.store[key] = value
}

func (s *StringStore) SafeSet(key, value string) {
    if _, ok := s.store[key]; !ok {
        s.store[key] = value
    }
}

func (s *StringStore) ReplaceSet(key, value string) string {
    val, ok := s.store[key]
    if ok {
        s.store[key] = value
        return val
    }

    s.store[key] = val
    return ""
}
