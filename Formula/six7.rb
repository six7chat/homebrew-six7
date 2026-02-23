class Six7 < Formula
  desc "Secure P2P chat CLI with Six7 Protocol v1.3 - powered by Korium 0.7.38"
  homepage "https://github.com/six7chat/homebrew-six7"
  version "0.7.38"
  license "MIT"

  on_macos do
    on_arm do
      url "https://github.com/six7chat/homebrew-six7/releases/download/v0.7.38/six7-0.7.38-arm64-apple-darwin.tar.gz"
      sha256 "7fe3546efbae74e5b8de6e5669c5e14f217befaaaa20c7397ebaeebf7fd4119f"
    end
  end

  def install
    bin.install "six7"
  end

  test do
    assert_match "six7", shell_output("#{bin}/six7 --version")
  end
end
